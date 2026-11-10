# Recorded comparison

Generated: 2026-11-10T03:00:00.461469+00:00

CPU: INTEL(R) XEON(R) PLATINUM 8573C. OS: Linux-6.18.57-x86_64-with-glibc2.41.
Rust: `rustc 1.99.1 (58cf8f9c3 2026-10-13)`. Pinned CPU: 7.

Release profile: opt-level 3, thin LTO, one codegen unit, stripped executables.

The JSON and CSV files contain every sample. A range is the observed minimum–maximum, not a confidence interval.

## Sparse long options

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 4 | zclap | 192.2 | 161.1–252.3 | 5,202,670 | 0 |
| 4 | clap | 2575.8 | 2293.3–3818.6 | 388,236 | 51 |
| 4 | lexopt | 336.0 | 270.8–499.6 | 2,976,509 | 7 |
| 4 | pico-args | 238.2 | 213.6–312.3 | 4,198,629 | 6 |
| 4 | clap_lex | 362.3 | 292.8–460.7 | 2,760,243 | 6 |
| 8 | zclap | 218.6 | 205.6–257.3 | 4,574,900 | 0 |
| 8 | clap | 4054.3 | 3564.8–5259.8 | 246,649 | 77 |
| 8 | lexopt | 339.0 | 309.4–461.5 | 2,949,504 | 8 |
| 8 | pico-args | 395.8 | 330.3–477.8 | 2,526,446 | 6 |
| 8 | clap_lex | 365.7 | 323.0–605.5 | 2,734,280 | 6 |
| 16 | zclap | 205.8 | 187.1–249.0 | 4,858,213 | 0 |
| 16 | clap | 6297.4 | 5567.6–15839.4 | 158,794 | 102 |
| 16 | lexopt | 313.2 | 270.3–382.7 | 3,193,001 | 7 |
| 16 | pico-args | 775.2 | 700.0–958.1 | 1,290,048 | 5 |
| 16 | clap_lex | 315.5 | 289.1–365.6 | 3,169,361 | 5 |
| 32 | zclap | 219.4 | 203.7–257.5 | 4,558,425 | 0 |
| 32 | clap | 14082.4 | 12611.1–16278.3 | 71,011 | 177 |
| 32 | lexopt | 348.6 | 311.1–394.1 | 2,868,872 | 8 |
| 32 | pico-args | 1222.4 | 1126.4–1557.9 | 818,037 | 6 |
| 32 | clap_lex | 352.9 | 324.9–392.7 | 2,833,359 | 6 |
| 64 | zclap | 239.4 | 206.2–294.7 | 4,177,197 | 0 |
| 64 | clap | 39961.1 | 31554.5–43587.6 | 25,024 | 298 |
| 64 | lexopt | 309.3 | 287.2–374.0 | 3,232,971 | 7 |
| 64 | pico-args | 2598.9 | 2447.0–3452.6 | 384,781 | 5 |
| 64 | clap_lex | 345.7 | 284.5–369.0 | 2,892,489 | 5 |
| 128 | zclap | 253.9 | 237.3–334.3 | 3,938,341 | 0 |
| 128 | clap | 120735.3 | 107237.8–155479.0 | 8,283 | 565 |
| 128 | lexopt | 350.9 | 318.0–385.0 | 2,849,595 | 8 |
| 128 | pico-args | 4282.5 | 3794.5–5023.0 | 233,511 | 6 |
| 128 | clap_lex | 349.5 | 318.2–464.9 | 2,861,099 | 6 |
| 256 | zclap | 264.9 | 229.1–400.9 | 3,774,383 | 0 |
| 256 | clap | 354444.5 | 315578.7–413542.9 | 2,821 | 1070 |
| 256 | lexopt | 308.4 | 286.4–387.4 | 3,242,479 | 7 |
| 256 | pico-args | 9785.6 | 9236.6–11119.5 | 102,191 | 5 |
| 256 | clap_lex | 322.3 | 292.1–448.5 | 3,102,767 | 5 |

## Dense long options

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 4 | zclap | 187.1 | 165.8–247.4 | 5,344,507 | 0 |
| 4 | clap | 2569.0 | 2194.2–4129.6 | 389,255 | 51 |
| 4 | lexopt | 323.5 | 272.8–451.7 | 3,090,732 | 7 |
| 4 | pico-args | 240.4 | 209.6–328.3 | 4,160,513 | 6 |
| 4 | clap_lex | 342.7 | 297.9–425.9 | 2,917,893 | 6 |
| 8 | zclap | 395.5 | 350.8–464.7 | 2,528,707 | 0 |
| 8 | clap | 6888.7 | 5689.6–8825.0 | 145,165 | 100 |
| 8 | lexopt | 618.4 | 560.0–819.6 | 1,617,160 | 13 |
| 8 | pico-args | 568.5 | 519.6–641.5 | 1,759,092 | 11 |
| 8 | clap_lex | 709.7 | 664.1–909.2 | 1,408,969 | 11 |
| 16 | zclap | 784.1 | 683.9–1633.8 | 1,275,291 | 0 |
| 16 | clap | 21003.3 | 16987.0–33124.2 | 47,612 | 194 |
| 16 | lexopt | 1262.0 | 1141.2–2594.1 | 792,369 | 25 |
| 16 | pico-args | 1317.9 | 1215.0–1547.1 | 758,801 | 21 |
| 16 | clap_lex | 1374.2 | 1215.4–1570.5 | 727,699 | 21 |
| 32 | zclap | 1675.5 | 1465.2–2011.7 | 596,820 | 0 |
| 32 | clap | 78615.9 | 73752.7–100833.0 | 12,720 | 378 |
| 32 | lexopt | 2506.9 | 2114.5–2832.5 | 398,901 | 49 |
| 32 | pico-args | 3787.1 | 3545.4–5657.8 | 264,052 | 41 |
| 32 | clap_lex | 2762.2 | 2477.0–3518.9 | 362,036 | 41 |
| 64 | zclap | 3336.5 | 3119.2–3871.2 | 299,716 | 0 |
| 64 | clap | 392577.1 | 367022.4–471708.6 | 2,547 | 742 |
| 64 | lexopt | 6826.6 | 6026.5–7753.4 | 146,485 | 97 |
| 64 | pico-args | 13688.6 | 12415.9–23083.7 | 73,053 | 81 |
| 64 | clap_lex | 7277.5 | 6323.5–16152.2 | 137,409 | 81 |
| 128 | zclap | 7381.6 | 6649.1–9463.4 | 135,471 | 0 |
| 128 | clap | 2197266.9 | 2031490.6–3011994.4 | 455 | 1466 |
| 128 | lexopt | 13110.9 | 11883.4–15085.3 | 76,272 | 193 |
| 128 | pico-args | 43501.2 | 40080.7–54295.3 | 22,988 | 161 |
| 128 | clap_lex | 13639.0 | 12421.9–58072.3 | 73,319 | 161 |
| 256 | zclap | 21161.9 | 18425.1–32643.5 | 47,255 | 0 |
| 256 | clap | 14611691.3 | 13120550.3–20362386.6 | 68 | 2910 |
| 256 | lexopt | 28455.7 | 23529.6–36388.1 | 35,142 | 385 |
| 256 | pico-args | 150281.3 | 135633.1–235474.6 | 6,654 | 321 |
| 256 | clap_lex | 28999.9 | 23424.2–50380.7 | 34,483 | 321 |

## Short flag cluster

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 5 | zclap | 118.3 | 98.8–194.4 | 8,456,159 | 0 |
| 5 | clap | 2640.2 | 1948.7–7612.3 | 378,757 | 51 |
| 5 | lexopt | 192.2 | 156.7–353.8 | 5,203,049 | 5 |
| 5 | pico-args | 377.9 | 329.5–712.2 | 2,646,504 | 6 |
| 5 | clap_lex | 155.9 | 124.6–302.7 | 6,413,751 | 5 |

## Repeated flags and values

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 5 | zclap | 293.4 | 256.9–455.3 | 3,408,119 | 0 |
| 5 | clap | 4633.3 | 3837.8–13497.5 | 215,827 | 102 |
| 5 | lexopt | 512.2 | 458.1–1766.2 | 1,952,237 | 12 |
| 5 | pico-args | 1524.2 | 1309.0–1975.7 | 656,095 | 11 |
| 5 | clap_lex | 464.9 | 387.8–622.2 | 2,150,787 | 9 |

## Mixed short and long syntax

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 5 | zclap | 213.5 | 176.5–601.9 | 4,684,455 | 0 |
| 5 | clap | 3558.5 | 3113.3–5170.2 | 281,018 | 74 |
| 5 | lexopt | 327.3 | 304.3–643.2 | 3,055,618 | 8 |
| 5 | pico-args | 708.2 | 591.9–1061.2 | 1,411,973 | 8 |
| 5 | clap_lex | 271.2 | 226.9–581.4 | 3,687,397 | 7 |

## Nested subcommands

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 5 | zclap | 185.7 | 168.1–635.3 | 5,384,537 | 0 |
| 5 | clap | 4078.3 | 3326.0–6193.3 | 245,200 | 87 |
| 5 | lexopt | 316.8 | 280.3–1310.3 | 3,156,217 | 8 |
| 5 | pico-args | 708.8 | 587.0–921.5 | 1,410,875 | 8 |
| 5 | clap_lex | 250.9 | 218.3–384.8 | 3,984,985 | 7 |

## Eight-byte command name

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 4 | zclap | 189.6 | 162.5–305.5 | 5,275,124 | 0 |
| 4 | clap | 2620.8 | 2246.4–3579.6 | 381,557 | 51 |
| 4 | lexopt | 323.9 | 263.8–819.0 | 3,087,458 | 7 |
| 4 | pico-args | 244.0 | 202.0–369.9 | 4,098,663 | 6 |
| 4 | clap_lex | 344.7 | 298.3–495.4 | 2,901,301 | 6 |

## 32-byte command name

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 4 | zclap | 181.9 | 165.1–222.2 | 5,496,922 | 0 |
| 4 | clap | 2682.2 | 2321.9–3713.8 | 372,822 | 51 |
| 4 | lexopt | 318.1 | 280.6–504.3 | 3,143,596 | 7 |
| 4 | pico-args | 235.1 | 213.5–304.4 | 4,254,215 | 6 |
| 4 | clap_lex | 320.3 | 297.3–1014.1 | 3,122,063 | 6 |

## 128-byte command name

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 4 | zclap | 191.1 | 158.7–622.8 | 5,231,576 | 0 |
| 4 | clap | 2535.7 | 2249.5–3594.0 | 394,368 | 51 |
| 4 | lexopt | 319.9 | 266.4–991.3 | 3,126,211 | 7 |
| 4 | pico-args | 242.4 | 216.8–333.7 | 4,126,008 | 6 |
| 4 | clap_lex | 349.5 | 303.8–555.0 | 2,861,091 | 6 |

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
