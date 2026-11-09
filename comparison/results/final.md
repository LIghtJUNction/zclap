# Recorded comparison

Generated: 2026-11-09T18:41:29.324719+00:00

CPU: INTEL(R) XEON(R) PLATINUM 8573C. OS: Linux-6.18.57-x86_64-with-glibc2.41.
Rust: `rustc 1.99.1 (58cf8f9c3 2026-10-13)`. Pinned CPU: 7.

Release profile: opt-level 3, thin LTO, one codegen unit, stripped executables.

The JSON and CSV files contain every sample. A range is the observed minimum–maximum, not a confidence interval.

## Sparse long options

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 4 | zclap | 162.9 | 156.8–199.4 | 6,139,225 | 0 |
| 4 | clap | 2284.9 | 2180.5–2409.1 | 437,662 | 51 |
| 4 | lexopt | 268.9 | 237.8–403.3 | 3,718,481 | 7 |
| 4 | pico-args | 203.7 | 182.1–271.0 | 4,908,530 | 6 |
| 4 | clap_lex | 275.0 | 255.9–304.7 | 3,636,946 | 6 |
| 8 | zclap | 190.9 | 176.7–269.8 | 5,238,509 | 0 |
| 8 | clap | 3718.0 | 3477.8–4630.2 | 268,965 | 77 |
| 8 | lexopt | 284.8 | 268.0–313.1 | 3,510,866 | 8 |
| 8 | pico-args | 311.8 | 302.1–345.0 | 3,207,493 | 6 |
| 8 | clap_lex | 311.0 | 266.1–338.0 | 3,215,817 | 6 |
| 16 | zclap | 186.3 | 177.7–201.2 | 5,368,292 | 0 |
| 16 | clap | 5670.1 | 5267.5–6625.1 | 176,362 | 102 |
| 16 | lexopt | 259.8 | 252.1–379.6 | 3,848,818 | 7 |
| 16 | pico-args | 690.4 | 619.5–803.6 | 1,448,423 | 5 |
| 16 | clap_lex | 275.6 | 255.9–379.7 | 3,628,394 | 5 |
| 32 | zclap | 209.8 | 199.8–289.4 | 4,766,376 | 0 |
| 32 | clap | 12897.5 | 11808.3–19872.3 | 77,534 | 177 |
| 32 | lexopt | 327.2 | 265.2–470.0 | 3,056,076 | 8 |
| 32 | pico-args | 1059.9 | 985.3–1589.0 | 943,477 | 6 |
| 32 | clap_lex | 286.5 | 273.8–409.7 | 3,490,133 | 6 |
| 64 | zclap | 269.0 | 198.2–374.4 | 3,716,975 | 0 |
| 64 | clap | 43083.7 | 35310.6–52322.6 | 23,211 | 298 |
| 64 | lexopt | 280.0 | 262.9–488.2 | 3,570,982 | 7 |
| 64 | pico-args | 2578.7 | 2160.0–4065.2 | 387,785 | 5 |
| 64 | clap_lex | 310.6 | 253.6–484.1 | 3,219,907 | 5 |
| 128 | zclap | 675.9 | 503.3–1717.3 | 1,479,542 | 0 |
| 128 | clap | 351360.3 | 266340.7–863958.4 | 2,846 | 565 |
| 128 | lexopt | 832.2 | 631.1–2299.7 | 1,201,637 | 8 |
| 128 | pico-args | 9803.2 | 7221.6–14923.5 | 102,008 | 6 |
| 128 | clap_lex | 998.7 | 606.1–2164.8 | 1,001,318 | 6 |
| 256 | zclap | 283.8 | 211.1–539.9 | 3,523,770 | 0 |
| 256 | clap | 353676.1 | 320442.7–679671.4 | 2,827 | 1070 |
| 256 | lexopt | 305.9 | 271.1–506.9 | 3,269,106 | 7 |
| 256 | pico-args | 9265.5 | 8353.7–61253.8 | 107,927 | 5 |
| 256 | clap_lex | 296.9 | 244.7–1000.1 | 3,368,115 | 5 |

## Dense long options

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 4 | zclap | 158.6 | 146.4–182.0 | 6,306,840 | 0 |
| 4 | clap | 2263.0 | 2201.1–2328.7 | 441,882 | 51 |
| 4 | lexopt | 252.1 | 225.0–272.4 | 3,965,988 | 7 |
| 4 | pico-args | 195.2 | 184.8–205.0 | 5,123,397 | 6 |
| 4 | clap_lex | 283.3 | 266.2–473.2 | 3,530,051 | 6 |
| 8 | zclap | 353.6 | 332.4–430.2 | 2,827,982 | 0 |
| 8 | clap | 5779.9 | 4753.0–13823.7 | 173,014 | 100 |
| 8 | lexopt | 605.3 | 464.7–713.2 | 1,652,008 | 13 |
| 8 | pico-args | 535.5 | 489.7–618.8 | 1,867,442 | 11 |
| 8 | clap_lex | 544.8 | 532.9–615.0 | 1,835,469 | 11 |
| 16 | zclap | 649.8 | 614.1–691.7 | 1,538,961 | 0 |
| 16 | clap | 17036.4 | 16639.3–19274.2 | 58,698 | 194 |
| 16 | lexopt | 1056.8 | 965.2–1252.1 | 946,264 | 25 |
| 16 | pico-args | 1168.8 | 1098.9–1308.1 | 855,604 | 21 |
| 16 | clap_lex | 1106.9 | 1054.8–1548.7 | 903,422 | 21 |
| 32 | zclap | 1511.5 | 1357.8–4060.9 | 661,594 | 0 |
| 32 | clap | 82856.9 | 65241.1–121468.4 | 12,069 | 378 |
| 32 | lexopt | 2325.7 | 2019.5–4018.8 | 429,972 | 49 |
| 32 | pico-args | 3617.0 | 3061.1–6026.2 | 276,470 | 41 |
| 32 | clap_lex | 2469.9 | 2158.7–6224.6 | 404,882 | 41 |
| 64 | zclap | 5995.2 | 2657.3–8957.0 | 166,799 | 0 |
| 64 | clap | 803389.8 | 374475.6–1530372.4 | 1,245 | 742 |
| 64 | lexopt | 17017.6 | 5446.7–30292.2 | 58,763 | 97 |
| 64 | pico-args | 27265.8 | 12154.5–45527.6 | 36,676 | 81 |
| 64 | clap_lex | 13896.3 | 6838.1–18149.5 | 71,962 | 81 |
| 128 | zclap | 20596.3 | 12900.5–95702.3 | 48,552 | 0 |
| 128 | clap | 7480673.7 | 4604803.7–21973926.1 | 134 | 1466 |
| 128 | lexopt | 44386.3 | 18113.3–98842.1 | 22,529 | 193 |
| 128 | pico-args | 158298.4 | 79530.0–237532.2 | 6,317 | 161 |
| 128 | clap_lex | 44900.9 | 26149.2–111267.2 | 22,271 | 161 |
| 256 | zclap | 19379.1 | 16341.8–24879.5 | 51,602 | 0 |
| 256 | clap | 14496540.8 | 11296299.6–18741132.7 | 69 | 2910 |
| 256 | lexopt | 25194.1 | 23261.7–48050.3 | 39,692 | 385 |
| 256 | pico-args | 159930.5 | 127346.7–303481.3 | 6,253 | 321 |
| 256 | clap_lex | 26022.7 | 23203.8–40133.0 | 38,428 | 321 |

## Minimal executable and clean compilation

Each executable implements the same four-option CLI and exit status. The size includes Rust's standard runtime.

| Parser | Stripped bytes ↓ | Median compile s ↓ | Min–max compile s |
|---|---:|---:|---:|
| zclap | 336,528 | 4.642 | 3.974–4.671 |
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

## Measured optimization

The baseline is commit `607fce2e62b4d926e9be315f95246e3de7be0650`. Both measurements use the same harness on CPU 7, seven rounds, a 100 ms target and at least 32 complete parses per sample. The optimized parser uses a const-built sorted name index for larger definitions and presence masks for final validation.

| Workload | Definitions | Baseline median ns | Optimized median ns | Median ratio |
|---|---:|---:|---:|---:|
| sparse | 4 | 156.2 | 162.9 | 0.96× |
| dense | 4 | 174.3 | 158.6 | 1.10× |
| sparse | 8 | 200.6 | 190.9 | 1.05× |
| dense | 8 | 357.1 | 353.6 | 1.01× |
| sparse | 16 | 299.8 | 186.3 | 1.61× |
| dense | 16 | 1041.4 | 649.8 | 1.60× |
| sparse | 32 | 431.6 | 209.8 | 2.06× |
| dense | 32 | 2669.0 | 1511.5 | 1.77× |
| sparse | 64 | 604.4 | 269.0 | 2.25× |
| dense | 64 | 9519.1 | 5995.2 | 1.59× |
| sparse | 128 | 1253.1 | 675.9 | 1.85× |
| dense | 128 | 32597.7 | 20596.3 | 1.58× |
| sparse | 256 | 2164.5 | 283.8 | 7.63× |
| dense | 256 | 119305.0 | 19379.1 | 6.16× |

At 256 definitions, the dense median falls from 119.305 µs to 19.379 µs (6.16×) and the sparse median from 2.165 µs to 0.284 µs (7.63×). The four-definition sparse median changes from 156.2 ns to 162.9 ns; the observed sample ranges overlap. Several 64/128-definition rounds contain large shared-host interruptions; retain the individual samples and avoid interpreting the reported ratios as universal speedups.

The optimized zclap median is the lowest among these five adapters in all 14 measured workloads. This is a statement about this common grammar and these samples. These libraries expose different feature sets, and the lexical adapters include explicit validation code.

## Evidence files

- [Optimized raw timings](optimized-timings.csv) and [allocation counts](optimized-allocations.csv).
- [Revised baseline](baseline-revised.json), including source hashes and every timing sample.
- [Competitor build records](competitor-builds.json) and [zclap build records](optimized.json), with three fresh-target compile samples each.
- [Combined machine-readable report](final.json), including exact versions, compiler, profile, source hashes and commands.

## Source revisions

- Baseline: `607fce2e62b4d926e9be315f95246e3de7be0650`.
- Optimized parser: `9d4854837641ddfb80eccd1f74b34eb926a5bf90`.
- The source-hash verification for each measured library file is in `final.json`; capture timestamps and the original dirty-worktree flag remain unchanged.
