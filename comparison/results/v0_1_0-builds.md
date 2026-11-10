# Recorded comparison

Generated: 2026-11-10T03:07:45.927856+00:00

CPU: INTEL(R) XEON(R) PLATINUM 8573C. OS: Linux-6.18.57-x86_64-with-glibc2.41.
Rust: `rustc 1.99.1 (58cf8f9c3 2026-10-13)`. Pinned CPU: 7.

Release profile: opt-level 3, thin LTO, one codegen unit, stripped executables.

The JSON and CSV files contain every sample. A range is the observed minimum–maximum, not a confidence interval.

## Minimal executable and clean compilation

Each executable implements the same four-option CLI and exit status. The size includes Rust's standard runtime.

| Parser | Stripped bytes ↓ | Median compile s ↓ | Min–max compile s |
|---|---:|---:|---:|
| zclap | 336,496 | 4.424 | 4.364–4.991 |

## Scope

- Shared cloud host; CPU affinity is used where supported; turbo and neighboring workloads are not controlled.
- Timing builds have no counting allocator. Allocation builds count alloc, alloc_zeroed and realloc calls separately from timing.
- Definitions are prepared once outside the timed loop. Every parse includes input ownership required by each API, semantic validation, result reading and drop.
- Build timings start with an empty target directory and use a warm local registry cache; network download time is excluded.
- The extended grammars use five leaf definitions: verbose, quiet, port, tag and a positional input; subcommands add two command levels.
- Repeated-flag inputs contain four occurrences, inside clap's u8 counter range. Inputs use UTF-8, allowed tag values, and no escaped option-looking positional values.
- Command-name cases vary only the command metadata name length (8/32/128 bytes); they share the same four-argument dense input.

The timed workloads use valid UTF-8 long options, short flag clusters, bounded repeated flags, repeated enum values, mixed syntax and nested subcommands. Help rendering, error formatting, operating-system process startup, non-UTF-8 values and escaped option-looking positional values are outside this common grammar.
Small-parser adapters perform the semantic checks absent from their lexer APIs. The harness rejects unknown arguments, forbidden repetitions, missing required arguments, invalid integers, invalid tag choices and invalid subcommands before collecting samples.
