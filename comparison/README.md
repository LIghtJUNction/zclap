# zclap comparison

This is an independent, unpublished Cargo workspace. Its dependencies never enter the published `zclap` package. `Cargo.lock` fixes the competitor versions: clap 4.6.7, lexopt 0.3.2, pico-args 0.5.0 and clap_lex 1.1.1.

## Recorded results

The [0.1.1 report](results/v0_1_1.md) compares the published 0.1.0 source, the new API/index implementation, and the final name-validation optimization. It contains 21 workloads, 15 samples per parser/workload, all five parsers, allocation counts, three clean compilation rounds, binary sizes and definition layouts. [Machine-readable evidence](results/v0_1_1.json) links the raw CSV/JSON and verifies measured source hashes against the three commits. The [original 0.1.0 report](results/final.md) remains unchanged as a historical result from an earlier session and protocol.

On this run, the final four-definition dense median is 187.1 ns versus 2569.0 ns for clap. Dense 16/64/256-definition cases improve 21.8%/44.1%/32.8% over the newly measured published 0.1.0 baseline. The dense four-definition, repeated-value and mixed cases have 8.6–10.8% higher medians than that baseline. All 21 zclap workloads request zero allocations. The report preserves every sample and range; overlapping ranges and shared-host variation limit causal claims about small differences.

The new `Command` layout is 440 bytes, down from 696; `Arg` is 160 bytes, up from 136. Consequently, the shallow footprint of one command plus its arguments decreases only for commands with at most ten arguments. These are measurements for the recorded compiler/target, excluding borrowed text and external slices.

## Workloads and validation

The 21 workloads consist of:

- Fourteen long-option cases: sparse and dense inputs at 4, 8, 16, 32, 64, 128 and 256 definitions. Half the definitions take an unsigned integer; half are non-repeating flags. The first value option is required. Sparse inputs select four definitions; dense inputs use every definition. Inputs mix separate and `=` values in a fixed non-definition order.
- Four cases using five leaf definitions: repeated `verbose`/`-v`, `quiet`/`-q`, required `port`/`-p`, repeated `tag`/`-t` chosen from `alpha`, `beta` and `gamma`, and a required positional input. Cases exercise a short flag cluster, repeated flags/values, mixed short/long/attached syntax, and nested `remote add` subcommands.
- Three focused cases with 8-, 32- and 128-byte command names. The argument definitions and four-definition dense input remain identical; only command metadata changes. These separate the name-validation cache from the other changes.

Every adapter consumes all supplied results into the same checksum and implements the same required, duplicate, unknown-option, missing-value and value-validation rules for the fixture grammar. The preflight compares 105 valid checksums and checks rejection of 1,025 invalid inputs. Run it alone with `cargo run --manifest-path comparison/Cargo.toml --release --locked --bin benchmark -- --check`.

The long-option adapters for `lexopt` and `clap_lex` use a prepared full-name hash table and a stack bitset to supply declaration validation. Their five-argument adapters implement the same small grammar directly. `pico-args` uses its key-query API and validates leftover tokens. All adapters' required conversion and validation work remains inside each measured parse.

## Measurement method

Command definitions and lookup tables are prepared outside timing. `clap` is built once and reused through `try_get_matches_from_mut`; `zclap` reuses its definition. All parsers start from the same borrowed string inputs. Timed work includes input ownership conversions required by each API, parsing, validation, reading results and destruction. This measures a borrowed-input caller, not a caller transferring already-owned operating-system argument buffers.

Rust's `black_box` obscures inputs, engine selection and result checksums. Parser order rotates between rounds. Calibration selects at least 32 complete parses per sample, even when this exceeds the requested duration. Latency executables use the ordinary system allocator. Separate executables count allocation and reallocation requests, keeping allocator instrumentation out of latency measurements. Requested bytes are not peak live memory. Throughput is `1e9 / median nanoseconds per complete parse`.

Minimal binaries implement the same four-option CLI without help generation. Each parser is built independently in a fresh target directory with an already downloaded registry cache. Build metrics retain all compile-time samples and stripped file sizes, including the common standard-library runtime. All use opt-level 3, thin LTO and one codegen unit. Downloads are outside the measured compile interval.

## Reproduce the current implementation

Use Python 3.11 or newer and a Rust compiler accepted by the pinned competitors. The recorded run used Rust 1.99.1; the main crate checks zclap's own MSRV separately. From the repository root:

```sh
python3 comparison/run.py --label local
```

Build products use `CARGO_TARGET_DIR` when set, otherwise a system temporary directory. Build jobs default to one. The runner builds and verifies both executables before measuring, then writes Markdown, JSON and raw CSV to `comparison/results/`. For shorter iterations, use `--skip-build-metrics --rounds 5 --millis 30`. For only clean builds and binary sizes, use `--metrics-only --compile-rounds 3`; `--build-parsers zclap` restricts those build measurements to zclap.

## Reproduce the interleaved revision comparison

Use the same expanded harness in all three checkouts. The following Linux shell commands create temporary worktrees at the exact measured source commits. Choose a CPU allowed on your host; the recorded run used CPU 7. Stop unrelated local compilation before the `compare.py` step.

```sh
bench_dir="$(mktemp -d)"
bench_cpu=7
export CARGO_TARGET_DIR="$bench_dir/target"
export CARGO_BUILD_JOBS=1

cargo fetch --manifest-path comparison/Cargo.toml --locked
git worktree add --detach "$bench_dir/baseline" 32df66134af7173db24d89cff6df4c44e9e3e5de
git worktree add --detach "$bench_dir/pre-cache" 9c5b011e6c76dfcd3b3d1b3c0fbb942a1df481e3
git worktree add --detach "$bench_dir/final" 5802aa5c14631dfae5bd8d77a812b47f71043bcd

for revision in baseline pre-cache final; do
    cp -R comparison/src "$bench_dir/$revision/comparison/"
    cp comparison/Cargo.toml comparison/Cargo.lock comparison/run.py comparison/compare.py \
        "$bench_dir/$revision/comparison/"
    cargo update --offline --manifest-path "$bench_dir/$revision/comparison/Cargo.toml" -p zclap
    python3 "$bench_dir/$revision/comparison/run.py" \
        --label "reproduce-$revision" --prepare-only --cpu "$bench_cpu"
done

python3 comparison/compare.py \
    "$bench_dir/baseline/comparison/results/reproduce-baseline-capture.json" \
    "$bench_dir/pre-cache/comparison/results/reproduce-pre-cache-capture.json" \
    "$bench_dir/final/comparison/results/reproduce-final-capture.json" \
    --passes 3 --rounds-per-pass 5 --millis 40 --cpu "$bench_cpu" \
    --label reproduce-interleaved

python3 "$bench_dir/baseline/comparison/run.py" \
    --label reproduce-baseline-builds --metrics-only --compile-rounds 3 \
    --build-parsers zclap --cpu "$bench_cpu"
python3 "$bench_dir/final/comparison/run.py" \
    --label reproduce-final-builds --metrics-only --compile-rounds 3 --cpu "$bench_cpu"
```

`cargo update --offline -p zclap` aligns each copied lockfile with that checkout's local package version while preserving the pinned competitors. This matters after the release bumps the main lockfile to 0.1.1: all three measured implementation commits still declare 0.1.0. The recorded captures were made before that bump and retain their original version metadata.

`--prepare-only` builds timing/allocation executables, runs semantic checks, records layouts and hashes, and rejects source changes during capture. `compare.py` verifies executable hashes, benchmark source equality, compiler/profile and competitor versions before executing the captured binaries. With three passes it rotates baseline/pre-cache/final, pre-cache/final/baseline, then final/baseline/pre-cache, yielding 15 samples per parser/workload. It records the actual schedule and timestamps. No compilation occurs in this step.

Interleaved CSV/JSON/Markdown output goes to the calling `comparison/results/`; build reports go to each worktree's `comparison/results/`. Captured executable paths are temporary local paths, so regenerate them on another machine. Preserve the resulting reports before removing the temporary worktrees and target directory.

## Limits

The common grammar uses UTF-8 values. Repeated-flag inputs contain four occurrences, within clap's `u8` counter range. Escaped option-looking positionals, non-UTF-8 operating-system values, help/error rendering and process startup are outside these timings; core tests cover the library behavior beyond the benchmark grammar.

The lexical libraries expose different abstractions. Their adapters include the work required by the shared grammar; the comparison does not imply equivalent feature sets. The clap comparison disables default features and enables only `std` and dynamic string definitions. A shared cloud CPU can be noisy even with affinity; the reports retain minimum, median, maximum and every sample without presenting the range as a confidence interval or claiming a universal speedup.
