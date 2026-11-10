# Recorded comparison

Generated: 2026-11-10T02:56:06.592349+00:00

CPU: INTEL(R) XEON(R) PLATINUM 8573C. OS: Linux-6.18.57-x86_64-with-glibc2.41.
Rust: `rustc 1.99.1 (58cf8f9c3 2026-10-13)`. Pinned CPU: 7.

Release profile: opt-level 3, thin LTO, one codegen unit, stripped executables.

The JSON and CSV files contain every sample. A range is the observed minimum–maximum, not a confidence interval.

## Sparse long options

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 4 | zclap | 166.0 | 154.3–290.6 | 6,025,766 | 0 |
| 4 | clap | 2592.3 | 2305.4–3829.4 | 385,752 | 51 |
| 4 | lexopt | 295.9 | 261.0–614.5 | 3,379,943 | 7 |
| 4 | pico-args | 221.6 | 196.2–313.9 | 4,511,923 | 6 |
| 4 | clap_lex | 319.8 | 274.3–497.9 | 3,127,189 | 6 |
| 8 | zclap | 213.1 | 195.1–311.3 | 4,691,730 | 0 |
| 8 | clap | 4285.6 | 3585.5–6047.3 | 233,337 | 77 |
| 8 | lexopt | 323.8 | 291.1–500.9 | 3,088,145 | 8 |
| 8 | pico-args | 358.5 | 321.4–513.1 | 2,789,750 | 6 |
| 8 | clap_lex | 328.0 | 300.6–1321.7 | 3,048,427 | 6 |
| 16 | zclap | 210.0 | 195.4–245.2 | 4,761,406 | 0 |
| 16 | clap | 6333.9 | 5782.1–6526.4 | 157,880 | 102 |
| 16 | lexopt | 299.5 | 270.8–451.2 | 3,338,397 | 7 |
| 16 | pico-args | 733.7 | 649.1–1049.1 | 1,362,894 | 5 |
| 16 | clap_lex | 295.0 | 260.6–348.3 | 3,389,922 | 5 |
| 32 | zclap | 228.8 | 206.8–256.4 | 4,370,133 | 0 |
| 32 | clap | 14073.3 | 13126.0–17511.6 | 71,057 | 177 |
| 32 | lexopt | 317.1 | 291.8–365.1 | 3,153,569 | 8 |
| 32 | pico-args | 1157.2 | 1052.4–1609.9 | 864,144 | 6 |
| 32 | clap_lex | 315.7 | 275.8–391.8 | 3,167,183 | 6 |
| 64 | zclap | 232.7 | 205.9–375.2 | 4,298,025 | 0 |
| 64 | clap | 42111.1 | 36873.0–47725.6 | 23,747 | 298 |
| 64 | lexopt | 296.5 | 261.4–362.7 | 3,373,239 | 7 |
| 64 | pico-args | 2593.9 | 2334.4–3372.3 | 385,526 | 5 |
| 64 | clap_lex | 290.7 | 248.7–447.3 | 3,439,487 | 5 |
| 128 | zclap | 261.9 | 234.4–399.3 | 3,818,747 | 0 |
| 128 | clap | 123402.1 | 107507.2–169232.6 | 8,104 | 565 |
| 128 | lexopt | 324.4 | 297.0–362.2 | 3,082,633 | 8 |
| 128 | pico-args | 4238.6 | 3752.0–5209.6 | 235,929 | 6 |
| 128 | clap_lex | 322.5 | 290.0–417.2 | 3,101,073 | 6 |
| 256 | zclap | 261.5 | 242.8–291.9 | 3,824,428 | 0 |
| 256 | clap | 387928.3 | 329465.0–485979.5 | 2,578 | 1070 |
| 256 | lexopt | 289.3 | 265.6–393.3 | 3,457,109 | 7 |
| 256 | pico-args | 9996.3 | 8611.0–12667.3 | 100,037 | 5 |
| 256 | clap_lex | 278.6 | 251.9–372.5 | 3,589,556 | 5 |

## Dense long options

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 4 | zclap | 169.2 | 154.8–210.6 | 5,908,664 | 0 |
| 4 | clap | 2646.6 | 2338.6–3369.0 | 377,843 | 51 |
| 4 | lexopt | 298.2 | 262.0–646.4 | 3,353,252 | 7 |
| 4 | pico-args | 222.8 | 200.3–288.9 | 4,488,411 | 6 |
| 4 | clap_lex | 307.3 | 274.9–401.4 | 3,253,630 | 6 |
| 8 | zclap | 373.7 | 320.0–490.3 | 2,675,993 | 0 |
| 8 | clap | 6457.9 | 5775.6–13187.5 | 154,850 | 100 |
| 8 | lexopt | 576.7 | 537.5–762.4 | 1,734,133 | 13 |
| 8 | pico-args | 511.3 | 476.2–664.3 | 1,955,765 | 11 |
| 8 | clap_lex | 641.6 | 577.8–1240.6 | 1,558,497 | 11 |
| 16 | zclap | 736.5 | 664.1–1244.6 | 1,357,775 | 0 |
| 16 | clap | 19286.1 | 17502.9–24047.0 | 51,851 | 194 |
| 16 | lexopt | 1196.7 | 1072.5–1336.6 | 835,623 | 25 |
| 16 | pico-args | 1270.1 | 1163.7–1692.4 | 787,336 | 21 |
| 16 | clap_lex | 1205.2 | 1075.9–1592.8 | 829,752 | 21 |
| 32 | zclap | 1646.1 | 1509.9–2089.6 | 607,509 | 0 |
| 32 | clap | 74801.7 | 69154.2–115949.1 | 13,369 | 378 |
| 32 | lexopt | 2282.0 | 2078.6–2714.6 | 438,215 | 49 |
| 32 | pico-args | 3836.6 | 3495.7–4333.7 | 260,647 | 41 |
| 32 | clap_lex | 2493.4 | 2255.5–3222.1 | 401,062 | 41 |
| 64 | zclap | 3416.6 | 2941.3–3924.1 | 292,692 | 0 |
| 64 | clap | 372097.7 | 347409.2–444549.6 | 2,687 | 742 |
| 64 | lexopt | 6531.9 | 5845.0–7824.2 | 153,094 | 97 |
| 64 | pico-args | 13528.8 | 12223.6–15759.0 | 73,917 | 81 |
| 64 | clap_lex | 6284.9 | 5831.1–9215.8 | 159,111 | 81 |
| 128 | zclap | 7480.4 | 6703.5–9176.4 | 133,683 | 0 |
| 128 | clap | 1999352.9 | 1785238.2–2782438.5 | 500 | 1466 |
| 128 | lexopt | 12835.4 | 12080.0–51503.8 | 77,910 | 193 |
| 128 | pico-args | 42583.2 | 38268.1–51431.5 | 23,483 | 161 |
| 128 | clap_lex | 13128.2 | 11692.0–16212.7 | 76,172 | 161 |
| 256 | zclap | 19159.0 | 17026.2–31120.3 | 52,195 | 0 |
| 256 | clap | 12576840.8 | 12303653.1–14157629.5 | 80 | 2910 |
| 256 | lexopt | 25247.8 | 22341.8–32310.4 | 39,607 | 385 |
| 256 | pico-args | 157299.2 | 138863.0–188936.0 | 6,357 | 321 |
| 256 | clap_lex | 25641.8 | 23042.4–29105.7 | 38,999 | 321 |

## Short flag cluster

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 5 | zclap | 123.2 | 113.0–152.4 | 8,115,763 | 0 |
| 5 | clap | 2301.7 | 2110.2–3104.1 | 434,452 | 51 |
| 5 | lexopt | 166.1 | 152.2–291.5 | 6,019,455 | 5 |
| 5 | pico-args | 348.2 | 303.7–437.7 | 2,872,185 | 6 |
| 5 | clap_lex | 137.9 | 127.4–164.8 | 7,253,315 | 5 |

## Repeated flags and values

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 5 | zclap | 265.7 | 241.7–343.6 | 3,763,742 | 0 |
| 5 | clap | 4369.6 | 3879.2–5281.9 | 228,853 | 102 |
| 5 | lexopt | 461.5 | 425.4–523.2 | 2,166,782 | 12 |
| 5 | pico-args | 1393.0 | 1258.7–1825.6 | 717,884 | 11 |
| 5 | clap_lex | 393.8 | 358.7–414.5 | 2,539,437 | 9 |

## Mixed short and long syntax

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 5 | zclap | 203.3 | 181.3–262.8 | 4,919,468 | 0 |
| 5 | clap | 3521.7 | 3152.0–4090.8 | 283,956 | 74 |
| 5 | lexopt | 306.1 | 273.9–368.9 | 3,267,162 | 8 |
| 5 | pico-args | 623.2 | 546.5–977.5 | 1,604,711 | 8 |
| 5 | clap_lex | 250.4 | 212.4–311.6 | 3,993,770 | 7 |

## Nested subcommands

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 5 | zclap | 206.1 | 183.1–245.6 | 4,852,555 | 0 |
| 5 | clap | 3958.5 | 3358.6–6607.5 | 252,619 | 87 |
| 5 | lexopt | 318.9 | 282.1–462.9 | 3,135,661 | 8 |
| 5 | pico-args | 618.4 | 537.6–765.8 | 1,617,139 | 8 |
| 5 | clap_lex | 219.3 | 194.4–362.6 | 4,559,402 | 7 |

## Eight-byte command name

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 4 | zclap | 174.9 | 159.2–260.4 | 5,718,959 | 0 |
| 4 | clap | 2539.4 | 2352.2–3582.8 | 393,790 | 51 |
| 4 | lexopt | 309.4 | 249.9–400.2 | 3,232,271 | 7 |
| 4 | pico-args | 217.4 | 189.8–355.3 | 4,599,774 | 6 |
| 4 | clap_lex | 302.1 | 269.4–421.1 | 3,310,568 | 6 |

## 32-byte command name

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 4 | zclap | 192.3 | 175.4–303.6 | 5,201,290 | 0 |
| 4 | clap | 2683.2 | 2173.8–3814.6 | 372,683 | 51 |
| 4 | lexopt | 313.1 | 252.6–467.9 | 3,194,225 | 7 |
| 4 | pico-args | 223.5 | 202.2–326.7 | 4,475,134 | 6 |
| 4 | clap_lex | 307.1 | 266.6–446.5 | 3,256,099 | 6 |

## 128-byte command name

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 4 | zclap | 285.7 | 260.2–431.4 | 3,500,175 | 0 |
| 4 | clap | 2548.8 | 2302.0–4769.4 | 392,349 | 51 |
| 4 | lexopt | 297.7 | 258.8–427.4 | 3,359,256 | 7 |
| 4 | pico-args | 217.2 | 196.9–302.6 | 4,604,963 | 6 |
| 4 | clap_lex | 305.6 | 266.7–497.7 | 3,271,716 | 6 |

## Definition layout

`Command`: 440 bytes. `Arg`: 160 bytes. Default `Parser`: 192 bytes.

The following shallow footprint is `size_of::<Command>() + n * size_of::<Arg>()`. It excludes borrowed name/help strings and external alias/rule slices; it describes this compiler and target.

| Argument definitions | Shallow definition bytes |
|---:|---:|
| 4 | 1080 |
| 8 | 1720 |
| 16 | 3000 |
| 32 | 5560 |
| 64 | 10680 |
| 128 | 20920 |
| 256 | 41400 |

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
