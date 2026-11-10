# Recorded comparison

Generated: 2026-11-10T03:10:28.366746+00:00

CPU: INTEL(R) XEON(R) PLATINUM 8573C. OS: Linux-6.18.57-x86_64-with-glibc2.41.
Rust: `rustc 1.99.1 (58cf8f9c3 2026-10-13)`. Pinned CPU: 7.

Release profile: opt-level 3, thin LTO, one codegen unit, stripped executables.

The JSON and CSV files contain every sample. A range is the observed minimum–maximum, not a confidence interval.

## Minimal executable and clean compilation

Each executable implements the same four-option CLI and exit status. The size includes Rust's standard runtime.

| Parser | Stripped bytes ↓ | Median compile s ↓ | Min–max compile s |
|---|---:|---:|---:|
| zclap | 336,560 | 4.902 | 4.839–5.530 |
| clap | 563,072 | 13.073 | 12.654–16.598 |
| lexopt | 338,968 | 4.655 | 4.544–4.703 |
| pico-args | 335,816 | 4.726 | 4.714–4.876 |
| clap_lex | 328,768 | 4.589 | 4.269–4.645 |

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
