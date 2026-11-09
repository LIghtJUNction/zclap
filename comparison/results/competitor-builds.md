# Recorded comparison

Generated: 2026-11-09T18:36:39.367816+00:00

CPU: INTEL(R) XEON(R) PLATINUM 8573C. OS: Linux-6.18.57-x86_64-with-glibc2.41.
Rust: `rustc 1.99.1 (58cf8f9c3 2026-10-13)`. Pinned CPU: 0.

Release profile: opt-level 3, thin LTO, one codegen unit, stripped executables.

The JSON and CSV files contain every sample. A range is the observed minimum–maximum, not a confidence interval.

## Minimal executable and clean compilation

Each executable implements the same four-option CLI and exit status. The size includes Rust's standard runtime.

| Parser | Stripped bytes ↓ | Median compile s ↓ | Min–max compile s |
|---|---:|---:|---:|
| clap | 563,072 | 10.430 | 9.006–22.227 |
| lexopt | 338,968 | 3.681 | 3.620–3.770 |
| pico-args | 335,816 | 3.430 | 3.229–4.074 |
| clap_lex | 328,768 | 3.472 | 3.431–3.726 |

## Scope

- Shared cloud host; CPU affinity is used where supported; turbo and neighboring workloads are not controlled.
- Timing builds have no counting allocator. Allocation builds count alloc, alloc_zeroed and realloc calls separately from timing.
- Definitions are prepared once outside the timed loop. Every parse includes input ownership required by each API, semantic validation, result reading and drop.
- Build timings start with an empty target directory and use a warm local registry cache; network download time is excluded.

The timed workloads use valid UTF-8 long options. They do not measure help rendering, error formatting, short clusters, subcommands, or operating-system process startup.
Small-parser adapters perform the semantic checks absent from their lexer APIs. The harness rejects unknown, repeated, missing, and invalid numeric values before collecting samples.
