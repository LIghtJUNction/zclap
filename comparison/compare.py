#!/usr/bin/env python3
"""Interleave captured revisions without compiling during the measurement window."""

import argparse
import copy
import csv
import datetime as dt
import io
import json
from pathlib import Path

import run as runner


def main():
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("captures", type=Path, nargs="+")
    cli.add_argument("--passes", type=int, default=3)
    cli.add_argument("--rounds-per-pass", type=int, default=3)
    cli.add_argument("--millis", type=int, default=60)
    cli.add_argument("--cpu", type=int, required=True)
    cli.add_argument("--label", default="interleaved")
    args = cli.parse_args()
    if args.passes < 1 or args.rounds_per_pass < 3 or args.millis < 1:
        cli.error("passes >= 1, rounds-per-pass >= 3 and millis >= 1 are required")
    if not all(c.isalnum() or c in "-_" for c in args.label):
        cli.error("label must contain only letters, digits, hyphens and underscores")
    reports = [json.loads(path.read_text()) for path in args.captures]
    labels = [report["parameters"]["label"] for report in reports]
    if len(labels) != len(set(labels)):
        cli.error("capture labels must be unique")
    first = reports[0]["metadata"]
    for report in reports[1:]:
        metadata = report["metadata"]
        for field in ["rustc", "cargo", "os", "cpu", "profile", "rustflags"]:
            if metadata[field] != first[field]:
                cli.error(f"captures differ in {field}")
        for path, digest in first["source_sha256"].items():
            if path.startswith("comparison/src/") and metadata["source_sha256"].get(path) != digest:
                cli.error(f"captures use different benchmark source: {path}")
        for package, version in first["packages"].items():
            if package not in {"zclap", "zclap-comparison"} and metadata["packages"].get(package) != version:
                cli.error(f"captures use different competitor version: {package}")
    samples = [[] for _ in reports]
    schedule = []
    for pass_index in range(args.passes):
        # Every revision occupies every position when passes equals revisions.
        order = [(position + pass_index) % len(reports) for position in range(len(reports))]
        for position, index in enumerate(order):
            started = dt.datetime.now(dt.timezone.utc).isoformat()
            print(f"Pass {pass_index + 1}/{args.passes}: {labels[index]}", flush=True)
            raw = runner.captured_command(reports[index], "timing", args.rounds_per_pass, args.millis, args.cpu)
            for row in csv.DictReader(io.StringIO(raw)):
                row["round"] = str(int(row["round"]) + pass_index * args.rounds_per_pass)
                row["pass"] = str(pass_index)
                row["pass_position"] = str(position)
                samples[index].append(row)
            schedule.append({"pass": pass_index, "position": position, "capture": labels[index],
                             "started_utc": started,
                             "finished_utc": dt.datetime.now(dt.timezone.utc).isoformat()})
    results = runner.HERE / "results"
    results.mkdir(exist_ok=True)
    for index, original in enumerate(reports):
        report = copy.deepcopy(original)
        label = labels[index]
        output = io.StringIO()
        writer = csv.DictWriter(output, fieldnames=list(samples[index][0]))
        writer.writeheader()
        writer.writerows(samples[index])
        raw = output.getvalue()
        (results / f"{label}-timings.csv").write_text(raw)
        report["timing"] = runner.summarize(raw)
        report["interleaved_measurement"] = {
            "cpu": args.cpu, "passes": args.passes, "rounds_per_pass": args.rounds_per_pass,
            "target_millis": args.millis, "schedule": schedule,
        }
        allocations = runner.captured_command(report, "allocations", 3, 1, args.cpu)
        (results / f"{label}-allocations.csv").write_text(allocations)
        report["allocations"] = list(csv.DictReader(io.StringIO(allocations)))
        (results / f"{label}.json").write_text(json.dumps(report, indent=2) + "\n")
        (results / f"{label}.md").write_text(runner.markdown(report))
    (results / f"{args.label}-schedule.json").write_text(json.dumps(schedule, indent=2) + "\n")
    print(f"Saved {len(reports)} revision reports in {results}", flush=True)


if __name__ == "__main__":
    main()
