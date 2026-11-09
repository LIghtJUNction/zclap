# zclap comparison

This is an independent, unpublished Cargo workspace. Its dependencies never enter the published `zclap` package. `Cargo.lock` fixes the exact competitor versions.

```sh
python3 comparison/run.py --label local
```

Build products use `CARGO_TARGET_DIR` when set, otherwise the system temporary directory.

Use Python 3.11 or newer, and a stable Rust compiler accepted by the pinned competitors. The `zclap` MSRV is checked in the main crate; competitor MSRVs can differ. For faster optimization iterations, use `--skip-build-metrics --rounds 5 --millis 30`.

## Recorded results

[Full results](results/final.md) include all seven definition sizes, both input densities, every parser, allocation counts, three clean compilation rounds and binary sizes. [Machine-readable data](results/final.json) records the measured source hashes and environment.

On the recorded Linux host, the optimized four-definition dense case takes 159 ns per complete parse versus 2263 ns for clap; zclap requests zero allocations in every measured case. At 256 definitions, the dense median improves 6.16× over the initial parser. Shared-host scheduling introduces large outliers in some cases; the full report preserves all ranges and states the workload limits.

## What is measured

The same valid token sequences are parsed with 4, 8, 16, 32, 64, 128 and 256 named option definitions. Half the definitions take one unsigned integer; half are non-repeating flags. The first value option is required. Sparse inputs select four definitions; dense inputs use all definitions. Inputs include separate and `=` values and a fixed non-definition order.

Every adapter rejects unknown options, duplicates, missing required options, missing values and invalid integers, then reads every supplied result into the same order-independent checksum. The preflight calls test those error classes and compare valid checksums. `lexopt` and `clap_lex` use a prepared full-name hash table and a stack bitset to supply declaration validation. `pico-args` uses its key-query API and validates leftover tokens.

Command definitions and lookup tables are prepared outside timing. `clap` is built once and reused through `try_get_matches_from_mut`; `zclap` reuses its definition. Timed work includes ownership conversions required by each parser API, parsing, validation, reading results and destruction. All parsers receive the same borrowed string inputs. Rust's `black_box` obscures inputs, engine selection and result checksums. Parser order rotates between rounds. Calibration selects at least 32 complete parses per sample, even when the requested sample duration is shorter.

Latency builds use the ordinary system allocator. A second build counts allocation and reallocation requests independently, avoiding instrumentation overhead in timing results. Throughput is `1e9 / median nanoseconds per complete parse`. Minimum, median, maximum and every individual round are retained in `results/`.

Minimal binaries implement the same four-option CLI without help generation. Each parser is built independently in a fresh target directory with an already downloaded registry cache. The report includes all compile-time samples and stripped file sizes, including the common standard-library runtime. All use opt-level 3, thin LTO and one codegen unit. Downloads are outside the measured compile interval.

## Limits

The measurements cover UTF-8 long-option workloads, including result consumption. Core tests cover short clusters, repeated values, subcommands and non-UTF-8 operating-system arguments, but these scenarios are not folded into this common-subset ranking. Help rendering and process startup are outside this benchmark. A shared cloud CPU can be noisy; the script pins one permitted CPU where available and records the observed range rather than claiming a confidence interval or universal speedup.

The lexical libraries expose different abstractions. Their adapters include the work needed for the shared grammar; the comparison does not imply equivalent feature sets. The `clap` comparison disables default features and enables only `std` and dynamic string definitions, so formatting and completion features do not inflate its measured work.
