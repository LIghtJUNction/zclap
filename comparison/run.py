#!/usr/bin/env python3
"""Reproduce timings, allocation counts, compile times, and binary sizes."""

import argparse
import csv
import datetime as dt
import io
import hashlib
import json
import os
from pathlib import Path
import platform
import shutil
import statistics
import subprocess
import tempfile
import time
import tomllib

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
TARGET = Path(os.environ.setdefault("CARGO_TARGET_DIR", str(Path(tempfile.gettempdir()) / "zclap-comparison-target")))
BINS = {
    "zclap": ("with-zclap", "size-zclap"),
    "clap": ("with-clap", "size-clap"),
    "lexopt": ("with-lexopt", "size-lexopt"),
    "pico-args": ("with-pico", "size-pico-args"),
    "clap_lex": ("with-clap-lex", "size-clap-lex"),
}


def command(args, *, env=None, cpu=None):
    affinity = None
    if cpu is not None and hasattr(os, "sched_setaffinity"):
        affinity = lambda: os.sched_setaffinity(0, {cpu})
    result = subprocess.run(args, cwd=HERE, text=True, capture_output=True,
                            env=env, preexec_fn=affinity, check=False)
    if result.returncode:
        raise RuntimeError(f"Command failed ({result.returncode}): {' '.join(map(str, args))}\n"
                           f"{result.stdout}\n{result.stderr}")
    return result.stdout


def metadata(cpu):
    cpu_name = platform.processor()
    cpuinfo = Path("/proc/cpuinfo")
    if cpuinfo.exists():
        for line in cpuinfo.read_text().splitlines():
            if line.startswith("model name"):
                cpu_name = line.split(":", 1)[1].strip()
                break
    return {
        "timestamp_utc": dt.datetime.now(dt.timezone.utc).isoformat(),
        "git_commit": command(["git", "rev-parse", "HEAD"]).strip(),
        "git_dirty": bool(command(["git", "status", "--porcelain"]).strip()),
        "source_sha256": {
            str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest()
            for p in sorted([ROOT / "Cargo.toml", HERE / "Cargo.toml", HERE / "Cargo.lock",
                             *ROOT.glob("src/*.rs"), *HERE.glob("src/**/*.rs")])
        },
        "rustc": command(["rustc", "-Vv"]).strip(),
        "cargo": command(["cargo", "-V"]).strip(),
        "os": platform.platform(),
        "cpu": cpu_name,
        "pinned_cpu": cpu,
        "build_target_directory": str(TARGET),
        "profile": {"opt_level": 3, "lto": "thin", "codegen_units": 1, "strip": True},
        "rustflags": os.environ.get("RUSTFLAGS", ""),
        "packages": {p["name"]: p["version"] for p in tomllib.loads((HERE / "Cargo.lock").read_text())["package"]},
        "notes": [
            "Shared cloud host; CPU affinity is used where supported; turbo and neighboring workloads are not controlled.",
            "Timing builds have no counting allocator. Allocation builds count alloc, alloc_zeroed and realloc calls separately from timing.",
            "Definitions are prepared once outside the timed loop. Every parse includes input ownership required by each API, semantic validation, result reading and drop.",
            "Build timings start with an empty target directory and use a warm local registry cache; network download time is excluded.",
        ],
    }


def summarize(csv_text):
    grouped = {}
    for row in csv.DictReader(io.StringIO(csv_text)):
        key = (row["scenario"], int(row["options"]), row["parser"])
        grouped.setdefault(key, []).append({
            "round": int(row["round"]), "iterations": int(row["iterations"]),
            "ns_per_parse": float(row["ns_per_parse"]),
            "parses_per_second": float(row["parses_per_second"]),
        })
    result = []
    for (scenario, options, parser), samples in grouped.items():
        values = [row["ns_per_parse"] for row in samples]
        median = statistics.median(values)
        result.append({"scenario": scenario, "options": options, "parser": parser,
                       "min_ns": min(values), "median_ns": median, "max_ns": max(values),
                       "median_parses_per_second": 1e9 / median,
                       "relative_range_percent": (max(values) - min(values)) / median * 100,
                       "samples": samples})
    return result


def measure_builds(rounds, parsers):
    results = []
    TARGET.mkdir(parents=True, exist_ok=True)
    for engine, (feature, binary) in BINS.items():
        if engine not in parsers:
            continue
        samples = []
        binary_bytes = None
        for _ in range(rounds):
            with tempfile.TemporaryDirectory(prefix="compile-", dir=TARGET) as directory:
                env = dict(os.environ, CARGO_TARGET_DIR=directory, CARGO_INCREMENTAL="0")
                start = time.perf_counter()
                command(["cargo", "build", "--locked", "--release", "--no-default-features",
                         "--features", feature, "--bin", binary], env=env)
                samples.append(time.perf_counter() - start)
                suffix = ".exe" if os.name == "nt" else ""
                executable = Path(directory) / "release" / (binary + suffix)
                binary_bytes = executable.stat().st_size
                actual = command([str(executable), "--opt000=1000", "--opt003", "--opt002", "1002"])
                expected = sum((i + 1) * 0x9E3779B9 + value for i, value in [(0, 1000), (3, 1), (2, 1002)])
                if actual.strip() != str(expected):
                    raise RuntimeError(f"{engine} size binary returned {actual!r}, expected {expected}")
                for invalid in [[], ["--opt000=1", "--opt000=2"], ["--opt000=1", "--bad"]]:
                    status = subprocess.run([str(executable), *invalid], cwd=HERE,
                                            capture_output=True, check=False).returncode
                    if status != 2:
                        raise RuntimeError(f"{engine} size binary returned {status} for {invalid!r}")
        results.append({"parser": engine, "binary_bytes": binary_bytes,
                        "min_compile_s": min(samples), "median_compile_s": statistics.median(samples),
                        "max_compile_s": max(samples), "compile_samples_s": samples})
        print(f"{engine}: {binary_bytes} bytes, median compile {statistics.median(samples):.3f} s", flush=True)
    return results


def markdown(report):
    lines = ["# Recorded comparison", "", f"Generated: {report['metadata']['timestamp_utc']}", "",
             f"CPU: {report['metadata']['cpu']}. OS: {report['metadata']['os']}.",
             f"Rust: `{report['metadata']['rustc'].splitlines()[0]}`. Pinned CPU: {report['metadata']['pinned_cpu']}.", "",
             "Release profile: opt-level 3, thin LTO, one codegen unit, stripped executables.", "",
             "The JSON and CSV files contain every sample. A range is the observed minimum–maximum, not a confidence interval.", ""]
    timing = report.get("timing", [])
    allocations = {(r["scenario"], int(r["options"]), r["parser"]): r for r in report.get("allocations", [])}
    for scenario in ["sparse", "dense"]:
        if not timing:
            break
        lines += [f"## {scenario.capitalize()} long options", "",
                  "| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |", 
                  "|---:|---|---:|---:|---:|---:|"]
        for row in sorted((r for r in timing if r["scenario"] == scenario),
                          key=lambda r: (r["options"], list(BINS).index(r["parser"]))):
            alloc = allocations.get((scenario, row["options"], row["parser"]), {}).get("allocations", "—")
            lines.append(f"| {row['options']} | {row['parser']} | {row['median_ns']:.1f} | "
                         f"{row['min_ns']:.1f}–{row['max_ns']:.1f} | {row['median_parses_per_second']:,.0f} | {alloc} |")
        lines.append("")
    if report.get("builds"):
        lines += ["## Minimal executable and clean compilation", "",
                  "Each executable implements the same four-option CLI and exit status. The size includes Rust's standard runtime.", "",
                  "| Parser | Stripped bytes ↓ | Median compile s ↓ | Min–max compile s |",
                  "|---|---:|---:|---:|"]
        for row in report["builds"]:
            lines.append(f"| {row['parser']} | {row['binary_bytes']:,} | {row['median_compile_s']:.3f} | "
                         f"{row['min_compile_s']:.3f}–{row['max_compile_s']:.3f} |")
        lines.append("")
    lines += ["## Scope", "", *[f"- {note}" for note in report["metadata"]["notes"]], "",
              "The timed workloads use valid UTF-8 long options. They do not measure help rendering, error formatting, short clusters, subcommands, or operating-system process startup.",
              "Small-parser adapters perform the semantic checks absent from their lexer APIs. The harness rejects unknown, repeated, missing, and invalid numeric values before collecting samples.", ""]
    return "\n".join(lines)


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("--label", default="latest")
    cli.add_argument("--rounds", type=int, default=7)
    cli.add_argument("--millis", type=int, default=40)
    cli.add_argument("--compile-rounds", type=int, default=3)
    cli.add_argument("--skip-build-metrics", action="store_true")
    cli.add_argument("--metrics-only", action="store_true")
    cli.add_argument("--cpu", type=int)
    cli.add_argument("--build-parsers", nargs="+", choices=list(BINS), default=list(BINS))
    args = cli.parse_args()
    if not all(c.isalnum() or c in "-_" for c in args.label):
        cli.error("label must contain only letters, digits, hyphens and underscores")
    if args.rounds < 3 or args.compile_rounds < 1 or args.millis < 1:
        cli.error("rounds >= 3, compile-rounds >= 1 and millis >= 1 are required")
    if shutil.which("cargo") is None:
        cli.error("cargo and rustc must be on PATH")
    cpu = args.cpu
    if cpu is None and hasattr(os, "sched_getaffinity"):
        cpu = min(os.sched_getaffinity(0))
    results = HERE / "results"
    results.mkdir(exist_ok=True)
    command(["cargo", "fetch", "--locked"])
    report = {"metadata": metadata(cpu), "parameters": vars(args)}
    if not args.metrics_only:
        command(["cargo", "build", "--locked", "--release", "--bin", "benchmark"])
        executable = TARGET / "release" / ("benchmark.exe" if os.name == "nt" else "benchmark")
        print("Measuring parse latency without allocation instrumentation…", flush=True)
        raw = command([str(executable), str(args.rounds), str(args.millis)], cpu=cpu)
        (results / f"{args.label}-timings.csv").write_text(raw)
        report["timing"] = summarize(raw)
        command(["cargo", "build", "--locked", "--release", "--features", "allocation-counting", "--bin", "benchmark"])
        print("Counting allocations in a separate build…", flush=True)
        raw = command([str(executable), str(args.rounds), str(args.millis)], cpu=cpu)
        (results / f"{args.label}-allocations.csv").write_text(raw)
        report["allocations"] = list(csv.DictReader(io.StringIO(raw)))
    if not args.skip_build_metrics:
        report["builds"] = measure_builds(args.compile_rounds, args.build_parsers)
    (results / f"{args.label}.json").write_text(json.dumps(report, indent=2) + "\n")
    (results / f"{args.label}.md").write_text(markdown(report))
    print(results / f"{args.label}.md", flush=True)


if __name__ == "__main__":
    main()
