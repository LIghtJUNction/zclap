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
os.environ.setdefault("CARGO_BUILD_JOBS", "1")
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
        "build_jobs": os.environ["CARGO_BUILD_JOBS"],
        "profile": {"opt_level": 3, "lto": "thin", "codegen_units": 1, "strip": True},
        "rustflags": os.environ.get("RUSTFLAGS", ""),
        "packages": {p["name"]: p["version"] for p in tomllib.loads((HERE / "Cargo.lock").read_text())["package"]},
        "notes": [
            "Shared cloud host; CPU affinity is used where supported; turbo and neighboring workloads are not controlled.",
            "Timing builds have no counting allocator. Allocation builds count alloc, alloc_zeroed and realloc calls separately from timing.",
            "Definitions are prepared once outside the timed loop. Every parse includes input ownership required by each API, semantic validation, result reading and drop.",
            "Build timings start with an empty target directory and use a warm local registry cache; network download time is excluded.",
            "The extended grammars use five leaf definitions: verbose, quiet, port, tag and a positional input; subcommands add two command levels.",
            "Repeated-flag inputs contain four occurrences, inside clap's u8 counter range. Inputs use UTF-8, allowed tag values, and no escaped option-looking positional values.",
            "Command-name cases vary only the command metadata name length (8/32/128 bytes); they share the same four-argument dense input.",
        ],
    }


def prepare(label, cpu):
    """Capture both executables before timing, so compilation can finish first."""
    captured = TARGET / "captured"
    captured.mkdir(parents=True, exist_ok=True)
    report = {"metadata": metadata(cpu)}
    suffix = ".exe" if os.name == "nt" else ""
    executable = TARGET / "release" / ("benchmark" + suffix)
    for mode, features in [("timing", []), ("allocations", ["--features", "allocation-counting"])]:
        command(["cargo", "build", "--locked", "--release", "--bin", "benchmark", *features])
        command([str(executable), "--check"], cpu=cpu)
        destination = captured / f"{label}-{mode}{suffix}"
        shutil.copy2(executable, destination)
        report[mode + "_executable"] = {
            "path": str(destination),
            "sha256": hashlib.sha256(destination.read_bytes()).hexdigest(),
        }
        if mode == "timing":
            report["layouts"] = json.loads(command([str(destination), "--layout"]))
    current = metadata(cpu)["source_sha256"]
    if current != report["metadata"]["source_sha256"]:
        raise RuntimeError("Source changed during executable capture; rerun after edits finish")
    report["definition_footprint"] = [
        {"definitions": count,
         "shallow_bytes": report["layouts"]["command_bytes"] + count * report["layouts"]["arg_bytes"]}
        for count in [4, 8, 16, 32, 64, 128, 256]
    ]
    return report


def captured_command(report, mode, rounds, millis, cpu):
    item = report[mode + "_executable"]
    path = Path(item["path"])
    if hashlib.sha256(path.read_bytes()).hexdigest() != item["sha256"]:
        raise RuntimeError(f"Captured {mode} executable changed: {path}")
    return command([str(path), str(rounds), str(millis)], cpu=cpu)


def summarize(csv_text):
    grouped = {}
    for row in csv.DictReader(io.StringIO(csv_text)):
        key = (row["scenario"], int(row["options"]), row["parser"])
        sample = {
            "round": int(row["round"]), "iterations": int(row["iterations"]),
            "ns_per_parse": float(row["ns_per_parse"]),
            "parses_per_second": float(row["parses_per_second"]),
        }
        for field in ["pass", "pass_position"]:
            if field in row:
                sample[field] = int(row[field])
        grouped.setdefault(key, []).append(sample)
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
    titles = {
        "sparse": "Sparse long options",
        "dense": "Dense long options",
        "short-cluster": "Short flag cluster",
        "repeats": "Repeated flags and values",
        "mixed": "Mixed short and long syntax",
        "subcommands": "Nested subcommands",
        "command-name-8": "Eight-byte command name",
        "command-name-32": "32-byte command name",
        "command-name-128": "128-byte command name",
    }
    for scenario in dict.fromkeys(row["scenario"] for row in timing):
        if not timing:
            break
        lines += [f"## {titles.get(scenario, scenario)}", "",
                  "| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |", 
                  "|---:|---|---:|---:|---:|---:|"]
        for row in sorted((r for r in timing if r["scenario"] == scenario),
                          key=lambda r: (r["options"], list(BINS).index(r["parser"]))):
            alloc = allocations.get((scenario, row["options"], row["parser"]), {}).get("allocations", "—")
            lines.append(f"| {row['options']} | {row['parser']} | {row['median_ns']:.1f} | "
                         f"{row['min_ns']:.1f}–{row['max_ns']:.1f} | {row['median_parses_per_second']:,.0f} | {alloc} |")
        lines.append("")
    if report.get("layouts"):
        lines += ["## Definition layout", "",
                  f"`Command`: {report['layouts']['command_bytes']} bytes. "
                  f"`Arg`: {report['layouts']['arg_bytes']} bytes. "
                  f"Default `Parser`: {report['layouts']['parser_bytes']} bytes.", "",
                  "The following shallow footprint is `size_of::<Command>() + n * size_of::<Arg>()`. "
                  "It excludes borrowed name/help strings and external alias/rule slices; it describes this compiler and target.", "",
                  "| Argument definitions | Shallow definition bytes |", "|---:|---:|"]
        for row in report["definition_footprint"]:
            lines.append(f"| {row['definitions']} | {row['shallow_bytes']} |")
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
              "The timed workloads use valid UTF-8 long options, short flag clusters, bounded repeated flags, repeated enum values, mixed syntax and nested subcommands. Help rendering, error formatting, operating-system process startup, non-UTF-8 values and escaped option-looking positional values are outside this common grammar.",
              "Small-parser adapters perform the semantic checks absent from their lexer APIs. The harness rejects unknown arguments, forbidden repetitions, missing required arguments, invalid integers, invalid tag choices and invalid subcommands before collecting samples.", ""]
    return "\n".join(lines)


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("--label", default="latest")
    cli.add_argument("--rounds", type=int, default=7)
    cli.add_argument("--millis", type=int, default=40)
    cli.add_argument("--compile-rounds", type=int, default=3)
    cli.add_argument("--skip-build-metrics", action="store_true")
    cli.add_argument("--metrics-only", action="store_true")
    cli.add_argument("--prepare-only", action="store_true")
    cli.add_argument("--prepared", type=Path, help="capture JSON produced by --prepare-only")
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
    if args.prepare_only and (args.prepared or args.metrics_only):
        cli.error("--prepare-only cannot be combined with --prepared or --metrics-only")
    if args.prepared:
        report = json.loads(args.prepared.read_text())
    elif args.metrics_only:
        report = {"metadata": metadata(cpu)}
    else:
        print("Building and checking both benchmark executables…", flush=True)
        report = prepare(args.label, cpu)
    report["parameters"] = {key: str(value) if isinstance(value, Path) else value for key, value in vars(args).items()}
    if args.prepare_only:
        path = results / f"{args.label}-capture.json"
        path.write_text(json.dumps(report, indent=2) + "\n")
        print(path, flush=True)
        return
    if not args.metrics_only:
        print("Measuring parse latency without allocation instrumentation…", flush=True)
        report["measurement_started_utc"] = dt.datetime.now(dt.timezone.utc).isoformat()
        raw = captured_command(report, "timing", args.rounds, args.millis, cpu)
        (results / f"{args.label}-timings.csv").write_text(raw)
        report["timing"] = summarize(raw)
        print("Counting allocations in a separate build…", flush=True)
        raw = captured_command(report, "allocations", args.rounds, args.millis, cpu)
        (results / f"{args.label}-allocations.csv").write_text(raw)
        report["allocations"] = list(csv.DictReader(io.StringIO(raw)))
    if not args.skip_build_metrics:
        report["builds"] = measure_builds(args.compile_rounds, args.build_parsers)
    (results / f"{args.label}.json").write_text(json.dumps(report, indent=2) + "\n")
    (results / f"{args.label}.md").write_text(markdown(report))
    print(results / f"{args.label}.md", flush=True)


if __name__ == "__main__":
    main()
