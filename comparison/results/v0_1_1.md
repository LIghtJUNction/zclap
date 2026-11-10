# zclap 0.1.1 measurements

The 0.1.1 implementation improves the measured dense 16-, 64- and 256-definition cases, adds shared short/repeated/subcommand coverage, and preserves zero allocation requests in all 21 workloads. Some small-workload medians are higher than the published 0.1.0 baseline; this report preserves those results and the observed variation.

## Sources and protocol

- Published 0.1.0 baseline: [`32df661`](https://github.com/LIghtJUNction/zclap/commit/32df66134af7173db24d89cff6df4c44e9e3e5de).
- New API and compact index, before the name-validation cache: [`9c5b011`](https://github.com/LIghtJUNction/zclap/commit/9c5b011e6c76dfcd3b3d1b3c0fbb942a1df481e3).
- Final implementation with cached name validation: [`5802aa5`](https://github.com/LIghtJUNction/zclap/commit/5802aa5c14631dfae5bd8d77a812b47f71043bcd).
- All four measured library source/manifest hashes match these commits. All three revisions use byte-identical benchmark Rust sources, the same pinned competitors and the same compiler/profile. Capture timestamps and dirty-worktree flags remain unchanged in the raw JSON.
- The implementation was measured before the release version bump, so captured Cargo package metadata still reads `0.1.0` for the 0.1.1 candidates.

Host: INTEL(R) XEON(R) PLATINUM 8573C; Linux-6.18.57-x86_64-with-glibc2.41. Compiler: `rustc 1.99.1 (58cf8f9c3 2026-10-13)`. CPU affinity: 7. Profile: opt-level 3, thin LTO, one codegen unit, stripped binaries.

Each revision has 15 samples per parser/workload: three passes of five rounds, a 40 ms sample target and at least 32 complete parses per sample. Revision order rotates across passes: baseline/pre-cache/final, pre-cache/final/baseline, final/baseline/pre-cache. All timing executables were built before measurement; local compiler jobs were paused. Allocations are counted afterward in separate executables. The shared host still introduces scheduling noise.

Every preflight checks 105 valid result checksums and 1,025 rejected invalid inputs. The full raw data has 1,575 timing rows per revision. Values below are wall-clock medians for complete parsing, validation, result consumption and destruction; definition construction and process startup are outside timing.

## Published baseline versus final implementation

All latencies are nanoseconds per complete parse. A positive change means the final median is slower. Ranges are observed minima and maxima, not confidence intervals.

| Workload | Definitions | 0.1.0 median ns | Final median ns | Change | Final min–max ns | Final parses/s |
|---|---:|---:|---:|---:|---:|---:|
| sparse | 4 | 171.7 | 192.2 | +11.9% | 161.1–252.3 | 5,202,670 |
| dense | 4 | 168.9 | 187.1 | +10.8% | 165.8–247.4 | 5,344,507 |
| sparse | 8 | 203.0 | 218.6 | +7.7% | 205.6–257.3 | 4,574,900 |
| dense | 8 | 367.5 | 395.5 | +7.6% | 350.8–464.7 | 2,528,707 |
| sparse | 16 | 204.4 | 205.8 | +0.7% | 187.1–249.0 | 4,858,213 |
| dense | 16 | 1002.5 | 784.1 | -21.8% | 683.9–1633.8 | 1,275,291 |
| sparse | 32 | 232.7 | 219.4 | -5.7% | 203.7–257.5 | 4,558,425 |
| dense | 32 | 2567.3 | 1675.5 | -34.7% | 1465.2–2011.7 | 596,820 |
| sparse | 64 | 240.0 | 239.4 | -0.3% | 206.2–294.7 | 4,177,197 |
| dense | 64 | 5971.1 | 3336.5 | -44.1% | 3119.2–3871.2 | 299,716 |
| sparse | 128 | 271.0 | 253.9 | -6.3% | 237.3–334.3 | 3,938,341 |
| dense | 128 | 14673.1 | 7381.6 | -49.7% | 6649.1–9463.4 | 135,471 |
| sparse | 256 | 269.7 | 264.9 | -1.8% | 229.1–400.9 | 3,774,383 |
| dense | 256 | 31489.6 | 21161.9 | -32.8% | 18425.1–32643.5 | 47,255 |
| short-cluster | 5 | 122.2 | 118.3 | -3.3% | 98.8–194.4 | 8,456,159 |
| repeats | 5 | 270.1 | 293.4 | +8.6% | 256.9–455.3 | 3,408,119 |
| mixed | 5 | 192.7 | 213.5 | +10.8% | 176.5–601.9 | 4,684,455 |
| subcommands | 5 | 192.4 | 185.7 | -3.5% | 168.1–635.3 | 5,384,537 |
| command-name-8 | 4 | 176.4 | 189.6 | +7.5% | 162.5–305.5 | 5,275,124 |
| command-name-32 | 4 | 190.2 | 181.9 | -4.4% | 165.1–222.2 | 5,496,922 |
| command-name-128 | 4 | 264.9 | 191.1 | -27.8% | 158.7–622.8 | 5,231,576 |

The dense 16/64/256-definition medians improve by 21.8%/44.1%/32.8% versus the published baseline. The dense four-definition, repeated-value and mixed cases show approximately 8.6–10.8% higher medians. Sample ranges overlap, and unchanged competitor controls also vary between revision passes; these results support the specific measured gains rather than a claim that every workload became faster.

## Separate name-validation optimization

This comparison holds the plural APIs and compact index constant. It moves command-name validation to definition construction, removing its repeated scan when creating a parser. The command-name cases have identical four-definition dense inputs and vary only the command metadata name length.

| Name length | Before cache ns | After cache ns | Median change | After/before ratios by pass |
|---:|---:|---:|---:|---|
| 8 | 174.9 | 189.6 | +8.4% | 1.075, 1.109, 1.030 |
| 32 | 192.3 | 181.9 | -5.4% | 0.777, 0.946, 0.955 |
| 128 | 285.7 | 191.1 | -33.1% | 0.612, 0.737, 0.651 |

The 128-byte-name improvement repeats in every pass: 38.8%, 26.3% and 34.9%. The 32-byte case also improves in all passes. The eight-byte case is slower in these samples, so the cache optimization is retained for its demonstrated longer-name benefit without claiming a universal short-name speedup.

## Competitors on representative final workloads

Latencies are microseconds per complete parse. All measured zclap workloads request zero allocations. Full per-parser allocation counts, requested bytes, throughput and ranges for every definition size and workload are in [the final run](v0_1_1-final.md) and its CSV/JSON files.

| Workload | Definitions | zclap µs | clap µs | lexopt µs | pico-args µs | clap_lex µs |
|---|---:|---:|---:|---:|---:|---:|
| dense | 4 | 0.187 | 2.569 | 0.324 | 0.240 | 0.343 |
| dense | 16 | 0.784 | 21.003 | 1.262 | 1.318 | 1.374 |
| dense | 64 | 3.336 | 392.577 | 6.827 | 13.689 | 7.278 |
| dense | 256 | 21.162 | 14,611.691 | 28.456 | 150.281 | 29.000 |
| short-cluster | 5 | 0.118 | 2.640 | 0.192 | 0.378 | 0.156 |
| repeats | 5 | 0.293 | 4.633 | 0.512 | 1.524 | 0.465 |
| mixed | 5 | 0.213 | 3.558 | 0.327 | 0.708 | 0.271 |
| subcommands | 5 | 0.186 | 4.078 | 0.317 | 0.709 | 0.251 |

## Memory tradeoff

On this compiler/target, `Command` shrinks from 696 to 440 bytes; `Arg` grows from 136 to 160 bytes to represent plural alias and relationship lists. The default `Parser` remains 192 bytes. For one command, the shallow definition footprint is smaller up to ten arguments and larger from eleven arguments onward.

The following totals are `size_of::<Command>() + n * size_of::<Arg>()`. Borrowed name/help strings and external alias/rule slices are excluded; these are measured Rust layouts, not an ABI guarantee.

| Arguments in one command | 0.1.0 bytes | Final bytes | Change bytes |
|---:|---:|---:|---:|
| 4 | 1,240 | 1,080 | -160 |
| 8 | 1,784 | 1,720 | -64 |
| 16 | 2,872 | 3,000 | +128 |
| 32 | 5,048 | 5,560 | +512 |
| 64 | 9,400 | 10,680 | +1,280 |
| 128 | 18,104 | 20,920 | +2,816 |
| 256 | 35,512 | 41,400 | +5,888 |

## Minimal binary and compilation

Each result has three independent builds with a fresh target directory and a warm registry cache. Compilation runs sequentially with one job. Each minimal binary implements the same four-option CLI, validates the same output, and returns exit code 2 for invalid input. The file sizes include the standard runtime.

| Version/parser | Stripped bytes | Median compile s | Min–max compile s |
|---|---:|---:|---:|
| 0.1.0 zclap | 336,496 | 4.424 | 4.364–4.991 |
| final zclap | 336,560 | 4.902 | 4.839–5.530 |
| clap | 563,072 | 13.073 | 12.654–16.598 |
| lexopt | 338,968 | 4.655 | 4.544–4.703 |
| pico-args | 335,816 | 4.726 | 4.714–4.876 |
| clap_lex | 328,768 | 4.589 | 4.269–4.645 |

The two observed zclap binaries differ by 64 bytes. Their checkout paths differ as well, so the whole file-size delta is not attributed solely to the parser changes. Compilation-time differences also retain their observed ranges.

## Grammar and interpretation

- Long-option tests retain all seven requested definition sizes, four selected definitions for sparse input and every definition for dense input. Half the options are unsigned integer values and half are non-repeating flags; the first value option is required.
- The four additional workloads use five leaf definitions: repeated verbose, quiet, required port, repeated tag chosen from alpha/beta/gamma, and a required positional input. The nested case enters `remote add` before parsing the leaf arguments.
- Repeated-flag inputs have four occurrences, within clap’s `u8` counter range. The common grammar uses UTF-8 inputs and excludes escaped option-looking positionals, help/error rendering, process startup and non-UTF-8 operating-system values. Core tests cover the library features outside this benchmark grammar.
- All adapters start from the same borrowed strings. Ownership-taking APIs construct their required owned inputs inside the measured call. This scope does not measure a caller transferring already-owned operating-system argument buffers.
- Command definitions and lookup tables are prepared outside timing. The lexical adapters implement the required validation explicitly. `clap` uses only its `std` and dynamic-string features; its definition is built once and reused.
- Allocation columns count allocator/reallocator requests and requested bytes, not peak live memory.

## Reproduction and evidence

Run the current implementation and all competitors with `python3 comparison/run.py --label local`. For an interleaved revision comparison, prepare each checkout with the same expanded harness using `--prepare-only`, then pass those captures to `compare.py`; [the comparison README](../README.md) gives the commands. Captured executable paths are local scratch paths; regenerate captures when reproducing on another machine.

- [Combined summary and source verification](v0_1_1.json).
- [0.1.0 expanded run](v0_1_0-expanded.json), [pre-cache run](v0_1_1-pre.json), and [final run](v0_1_1-final.json).
- [Final raw timings](v0_1_1-final-timings.csv), [final allocation counts](v0_1_1-final-allocations.csv), and [interleaving schedule](v0_1_1-interleaved-schedule.json).
- [Baseline build samples](v0_1_0-builds.json) and [final/competitor build samples](v0_1_1-builds.json).
- The original 0.1.0 result files remain unchanged.
