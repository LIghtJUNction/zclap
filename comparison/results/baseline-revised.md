# Recorded comparison

Generated: 2026-11-09T18:39:02.746581+00:00

CPU: INTEL(R) XEON(R) PLATINUM 8573C. OS: Linux-6.18.57-x86_64-with-glibc2.41.
Rust: `rustc 1.99.1 (58cf8f9c3 2026-10-13)`. Pinned CPU: 7.

Release profile: opt-level 3, thin LTO, one codegen unit, stripped executables.

The JSON and CSV files contain every sample. A range is the observed minimum–maximum, not a confidence interval.

## Sparse long options

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 4 | zclap | 156.2 | 144.4–173.9 | 6,403,032 | 0 |
| 4 | clap | 2384.7 | 2312.4–2567.0 | 419,347 | 51 |
| 4 | lexopt | 265.0 | 248.4–408.7 | 3,773,485 | 7 |
| 4 | pico-args | 214.6 | 176.4–261.7 | 4,659,593 | 6 |
| 4 | clap_lex | 287.0 | 266.6–294.3 | 3,484,770 | 6 |
| 8 | zclap | 200.6 | 190.0–230.6 | 4,983,952 | 0 |
| 8 | clap | 3725.8 | 3630.9–4540.5 | 268,402 | 77 |
| 8 | lexopt | 282.8 | 268.4–305.8 | 3,535,868 | 8 |
| 8 | pico-args | 332.4 | 308.6–347.5 | 3,008,677 | 6 |
| 8 | clap_lex | 285.8 | 251.3–367.7 | 3,499,097 | 6 |
| 16 | zclap | 299.8 | 245.4–364.0 | 3,335,068 | 0 |
| 16 | clap | 6912.5 | 6036.0–7875.0 | 144,664 | 102 |
| 16 | lexopt | 382.5 | 280.0–432.1 | 2,614,482 | 7 |
| 16 | pico-args | 765.1 | 672.5–932.7 | 1,307,036 | 5 |
| 16 | clap_lex | 314.8 | 281.2–391.3 | 3,176,156 | 5 |
| 32 | zclap | 431.6 | 380.7–594.7 | 2,316,783 | 0 |
| 32 | clap | 13630.5 | 13168.5–16076.8 | 73,365 | 177 |
| 32 | lexopt | 310.8 | 297.3–458.3 | 3,217,296 | 8 |
| 32 | pico-args | 1148.0 | 1084.5–1383.4 | 871,071 | 6 |
| 32 | clap_lex | 302.6 | 288.4–324.8 | 3,304,496 | 6 |
| 64 | zclap | 604.4 | 576.6–729.8 | 1,654,460 | 0 |
| 64 | clap | 37923.8 | 33663.8–39826.1 | 26,369 | 298 |
| 64 | lexopt | 267.0 | 256.9–411.3 | 3,745,038 | 7 |
| 64 | pico-args | 2378.4 | 2264.3–2423.7 | 420,458 | 5 |
| 64 | clap_lex | 265.3 | 244.4–283.3 | 3,768,778 | 5 |
| 128 | zclap | 1253.1 | 1084.5–3254.6 | 798,008 | 0 |
| 128 | clap | 108520.6 | 96018.9–142530.7 | 9,215 | 565 |
| 128 | lexopt | 300.6 | 288.6–344.1 | 3,326,182 | 8 |
| 128 | pico-args | 4154.3 | 3626.9–6519.4 | 240,716 | 6 |
| 128 | clap_lex | 269.9 | 241.4–631.9 | 3,705,556 | 6 |
| 256 | zclap | 2164.5 | 2064.8–2369.8 | 462,007 | 0 |
| 256 | clap | 336581.6 | 327362.6–384768.3 | 2,971 | 1070 |
| 256 | lexopt | 270.3 | 243.3–368.7 | 3,699,265 | 7 |
| 256 | pico-args | 8873.2 | 8576.2–14626.9 | 112,699 | 5 |
| 256 | clap_lex | 258.2 | 252.9–315.3 | 3,873,087 | 5 |

## Dense long options

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 4 | zclap | 174.3 | 155.5–204.8 | 5,735,787 | 0 |
| 4 | clap | 2237.9 | 2177.2–2458.7 | 446,841 | 51 |
| 4 | lexopt | 259.8 | 248.9–306.5 | 3,849,648 | 7 |
| 4 | pico-args | 215.0 | 181.4–297.1 | 4,651,877 | 6 |
| 4 | clap_lex | 272.3 | 261.2–389.7 | 3,672,096 | 6 |
| 8 | zclap | 357.1 | 327.8–592.9 | 2,800,320 | 0 |
| 8 | clap | 5993.3 | 5313.2–11075.5 | 166,853 | 100 |
| 8 | lexopt | 508.2 | 492.5–589.3 | 1,967,772 | 13 |
| 8 | pico-args | 494.2 | 473.6–542.0 | 2,023,632 | 11 |
| 8 | clap_lex | 601.1 | 560.2–923.9 | 1,663,489 | 11 |
| 16 | zclap | 1041.4 | 892.6–1274.1 | 960,253 | 0 |
| 16 | clap | 20621.6 | 17750.0–25036.1 | 48,493 | 194 |
| 16 | lexopt | 1311.8 | 992.3–1638.6 | 762,286 | 25 |
| 16 | pico-args | 1401.0 | 1263.9–1847.4 | 713,761 | 21 |
| 16 | clap_lex | 1177.5 | 1134.1–1546.6 | 849,257 | 21 |
| 32 | zclap | 2669.0 | 2625.2–3202.4 | 374,670 | 0 |
| 32 | clap | 68576.4 | 66007.3–85225.2 | 14,582 | 378 |
| 32 | lexopt | 2135.9 | 1781.5–2856.2 | 468,178 | 49 |
| 32 | pico-args | 3491.1 | 3378.1–3858.0 | 286,441 | 41 |
| 32 | clap_lex | 2206.9 | 2087.9–2681.4 | 453,119 | 41 |
| 64 | zclap | 9519.1 | 9140.5–13358.9 | 105,052 | 0 |
| 64 | clap | 346950.7 | 312564.5–421183.5 | 2,882 | 742 |
| 64 | lexopt | 5654.8 | 5467.7–10716.5 | 176,841 | 97 |
| 64 | pico-args | 12377.6 | 11561.1–13454.8 | 80,791 | 81 |
| 64 | clap_lex | 6136.3 | 5458.5–6588.4 | 162,963 | 81 |
| 128 | zclap | 32597.7 | 31204.4–227820.9 | 30,677 | 0 |
| 128 | clap | 1746178.0 | 1672450.3–2844057.3 | 573 | 1466 |
| 128 | lexopt | 11301.9 | 10778.1–82913.3 | 88,481 | 193 |
| 128 | pico-args | 42610.4 | 37517.5–1870266.8 | 23,468 | 161 |
| 128 | clap_lex | 12249.8 | 11102.1–33344.2 | 81,634 | 161 |
| 256 | zclap | 119305.0 | 116854.1–215456.7 | 8,382 | 0 |
| 256 | clap | 10883841.7 | 10617543.9–11296994.8 | 92 | 2910 |
| 256 | lexopt | 23323.1 | 21452.1–26986.9 | 42,876 | 385 |
| 256 | pico-args | 146180.3 | 128496.5–167863.4 | 6,841 | 321 |
| 256 | clap_lex | 22931.6 | 21547.2–35034.8 | 43,608 | 321 |

## Scope

- Shared cloud host; CPU affinity is used where supported; turbo and neighboring workloads are not controlled.
- Timing builds have no counting allocator. Allocation builds count alloc, alloc_zeroed and realloc calls separately from timing.
- Definitions are prepared once outside the timed loop. Every parse includes input ownership required by each API, semantic validation, result reading and drop.
- Build timings start with an empty target directory and use a warm local registry cache; network download time is excluded.

The timed workloads use valid UTF-8 long options. They do not measure help rendering, error formatting, short clusters, subcommands, or operating-system process startup.
Small-parser adapters perform the semantic checks absent from their lexer APIs. The harness rejects unknown, repeated, missing, and invalid numeric values before collecting samples.
