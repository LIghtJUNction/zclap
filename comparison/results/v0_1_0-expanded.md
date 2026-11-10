# Recorded comparison

Generated: 2026-11-10T02:56:41.704589+00:00

CPU: INTEL(R) XEON(R) PLATINUM 8573C. OS: Linux-6.18.57-x86_64-with-glibc2.41.
Rust: `rustc 1.99.1 (58cf8f9c3 2026-10-13)`. Pinned CPU: 7.

Release profile: opt-level 3, thin LTO, one codegen unit, stripped executables.

The JSON and CSV files contain every sample. A range is the observed minimum–maximum, not a confidence interval.

## Sparse long options

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 4 | zclap | 171.7 | 158.3–179.4 | 5,823,196 | 0 |
| 4 | clap | 2513.8 | 2301.8–10082.7 | 397,798 | 51 |
| 4 | lexopt | 281.7 | 262.5–341.5 | 3,550,254 | 7 |
| 4 | pico-args | 219.7 | 192.1–239.9 | 4,552,490 | 6 |
| 4 | clap_lex | 319.1 | 284.6–1303.8 | 3,134,050 | 6 |
| 8 | zclap | 203.0 | 176.8–233.1 | 4,925,502 | 0 |
| 8 | clap | 4140.7 | 3673.1–4923.1 | 241,504 | 77 |
| 8 | lexopt | 326.3 | 294.3–372.0 | 3,065,059 | 8 |
| 8 | pico-args | 346.8 | 307.8–389.3 | 2,883,315 | 6 |
| 8 | clap_lex | 330.5 | 301.4–405.0 | 3,025,810 | 6 |
| 16 | zclap | 204.4 | 185.3–284.0 | 4,893,445 | 0 |
| 16 | clap | 6270.3 | 5421.3–7045.4 | 159,483 | 102 |
| 16 | lexopt | 315.8 | 292.7–824.3 | 3,166,351 | 7 |
| 16 | pico-args | 736.6 | 673.5–947.2 | 1,357,648 | 5 |
| 16 | clap_lex | 307.3 | 272.3–532.5 | 3,254,668 | 5 |
| 32 | zclap | 232.7 | 212.8–790.4 | 4,297,397 | 0 |
| 32 | clap | 13803.2 | 12320.1–16044.8 | 72,447 | 177 |
| 32 | lexopt | 328.9 | 301.7–1306.2 | 3,040,595 | 8 |
| 32 | pico-args | 1235.6 | 1130.0–1529.5 | 809,340 | 6 |
| 32 | clap_lex | 352.3 | 316.9–375.8 | 2,838,707 | 6 |
| 64 | zclap | 240.0 | 211.9–307.9 | 4,165,990 | 0 |
| 64 | clap | 40882.2 | 36229.9–52214.0 | 24,461 | 298 |
| 64 | lexopt | 313.1 | 274.6–361.7 | 3,193,847 | 7 |
| 64 | pico-args | 2641.3 | 2350.7–9762.4 | 378,595 | 5 |
| 64 | clap_lex | 304.8 | 271.8–354.6 | 3,281,174 | 5 |
| 128 | zclap | 271.0 | 238.3–746.7 | 3,689,533 | 0 |
| 128 | clap | 123345.4 | 104332.0–141629.3 | 8,107 | 565 |
| 128 | lexopt | 347.9 | 322.8–390.2 | 2,874,745 | 8 |
| 128 | pico-args | 4244.7 | 3775.1–5112.3 | 235,588 | 6 |
| 128 | clap_lex | 341.7 | 309.6–399.5 | 2,926,295 | 6 |
| 256 | zclap | 269.7 | 250.3–990.7 | 3,707,755 | 0 |
| 256 | clap | 358649.6 | 343602.7–405214.8 | 2,788 | 1070 |
| 256 | lexopt | 302.5 | 258.4–362.3 | 3,305,250 | 7 |
| 256 | pico-args | 9588.2 | 7761.9–11594.6 | 104,295 | 5 |
| 256 | clap_lex | 317.8 | 279.9–358.0 | 3,146,188 | 5 |

## Dense long options

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 4 | zclap | 168.9 | 153.6–407.1 | 5,921,750 | 0 |
| 4 | clap | 2595.0 | 2333.4–2974.8 | 385,360 | 51 |
| 4 | lexopt | 296.7 | 273.2–358.1 | 3,370,533 | 7 |
| 4 | pico-args | 212.8 | 191.5–248.3 | 4,698,387 | 6 |
| 4 | clap_lex | 324.0 | 293.9–405.2 | 3,086,305 | 6 |
| 8 | zclap | 367.5 | 324.6–409.4 | 2,720,763 | 0 |
| 8 | clap | 6529.9 | 5952.0–9170.0 | 153,141 | 100 |
| 8 | lexopt | 588.2 | 542.2–1819.3 | 1,700,099 | 13 |
| 8 | pico-args | 498.6 | 459.4–2723.9 | 2,005,580 | 11 |
| 8 | clap_lex | 697.5 | 627.9–917.0 | 1,433,675 | 11 |
| 16 | zclap | 1002.5 | 877.9–1122.8 | 997,476 | 0 |
| 16 | clap | 20693.6 | 17966.5–26502.5 | 48,324 | 194 |
| 16 | lexopt | 1208.2 | 1073.8–1725.4 | 827,689 | 25 |
| 16 | pico-args | 1229.2 | 1139.2–1377.5 | 813,541 | 21 |
| 16 | clap_lex | 1332.2 | 1155.4–1820.5 | 750,631 | 21 |
| 32 | zclap | 2567.3 | 2225.0–2989.4 | 389,508 | 0 |
| 32 | clap | 78311.2 | 68508.4–289531.9 | 12,770 | 378 |
| 32 | lexopt | 2345.0 | 2005.4–2759.9 | 426,432 | 49 |
| 32 | pico-args | 3659.6 | 3219.3–4552.1 | 273,257 | 41 |
| 32 | clap_lex | 2676.8 | 2487.3–4243.6 | 373,574 | 41 |
| 64 | zclap | 5971.1 | 5307.3–11299.6 | 167,474 | 0 |
| 64 | clap | 399549.5 | 349538.3–496701.2 | 2,503 | 742 |
| 64 | lexopt | 6316.7 | 5918.3–7302.8 | 158,309 | 97 |
| 64 | pico-args | 13276.3 | 12056.9–15009.9 | 75,322 | 81 |
| 64 | clap_lex | 6864.1 | 5563.7–9024.1 | 145,685 | 81 |
| 128 | zclap | 14673.1 | 13198.3–15909.0 | 68,152 | 0 |
| 128 | clap | 2102738.6 | 2042703.4–2274898.0 | 476 | 1466 |
| 128 | lexopt | 12713.7 | 11540.4–13940.7 | 78,655 | 193 |
| 128 | pico-args | 41721.2 | 40042.4–46497.1 | 23,969 | 161 |
| 128 | clap_lex | 13389.8 | 11841.8–18204.8 | 74,683 | 161 |
| 256 | zclap | 31489.6 | 27628.8–35076.2 | 31,757 | 0 |
| 256 | clap | 13375196.2 | 13108595.2–14869778.3 | 75 | 2910 |
| 256 | lexopt | 26065.2 | 22286.1–28558.7 | 38,365 | 385 |
| 256 | pico-args | 151843.9 | 138039.8–164359.8 | 6,586 | 321 |
| 256 | clap_lex | 25971.2 | 23735.9–33240.0 | 38,504 | 321 |

## Short flag cluster

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 5 | zclap | 122.2 | 102.1–137.4 | 8,180,227 | 0 |
| 5 | clap | 2253.8 | 2016.7–4882.5 | 443,693 | 51 |
| 5 | lexopt | 171.1 | 157.2–431.6 | 5,843,033 | 5 |
| 5 | pico-args | 339.9 | 310.6–468.1 | 2,941,748 | 6 |
| 5 | clap_lex | 131.6 | 120.5–147.2 | 7,596,360 | 5 |

## Repeated flags and values

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 5 | zclap | 270.1 | 237.7–311.3 | 3,702,607 | 0 |
| 5 | clap | 4367.8 | 3926.4–8350.5 | 228,950 | 102 |
| 5 | lexopt | 504.6 | 422.8–589.1 | 1,981,964 | 12 |
| 5 | pico-args | 1451.4 | 1289.3–1646.4 | 689,006 | 11 |
| 5 | clap_lex | 432.6 | 400.6–477.6 | 2,311,781 | 9 |

## Mixed short and long syntax

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 5 | zclap | 192.7 | 169.6–215.1 | 5,189,360 | 0 |
| 5 | clap | 3442.3 | 3115.9–4066.0 | 290,504 | 74 |
| 5 | lexopt | 301.6 | 280.6–347.4 | 3,315,144 | 8 |
| 5 | pico-args | 641.6 | 579.2–724.0 | 1,558,606 | 8 |
| 5 | clap_lex | 244.4 | 230.5–277.6 | 4,092,289 | 7 |

## Nested subcommands

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 5 | zclap | 192.4 | 176.5–325.7 | 5,197,046 | 0 |
| 5 | clap | 3802.6 | 3496.9–9719.2 | 262,976 | 87 |
| 5 | lexopt | 311.1 | 266.9–376.5 | 3,214,401 | 8 |
| 5 | pico-args | 649.2 | 579.5–932.4 | 1,540,279 | 8 |
| 5 | clap_lex | 229.5 | 203.6–268.3 | 4,356,805 | 7 |

## Eight-byte command name

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 4 | zclap | 176.4 | 155.3–210.4 | 5,668,709 | 0 |
| 4 | clap | 2498.3 | 2261.0–3942.8 | 400,266 | 51 |
| 4 | lexopt | 300.6 | 260.8–317.2 | 3,326,580 | 7 |
| 4 | pico-args | 215.6 | 192.5–343.1 | 4,637,488 | 6 |
| 4 | clap_lex | 326.6 | 285.9–478.8 | 3,061,681 | 6 |

## 32-byte command name

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 4 | zclap | 190.2 | 167.8–225.6 | 5,257,264 | 0 |
| 4 | clap | 2587.8 | 2288.1–6433.9 | 386,436 | 51 |
| 4 | lexopt | 287.4 | 268.6–343.5 | 3,479,290 | 7 |
| 4 | pico-args | 216.2 | 187.9–250.2 | 4,624,598 | 6 |
| 4 | clap_lex | 322.1 | 293.3–368.8 | 3,104,944 | 6 |

## 128-byte command name

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 4 | zclap | 264.9 | 245.7–321.7 | 3,774,696 | 0 |
| 4 | clap | 2514.7 | 2273.0–13953.5 | 397,658 | 51 |
| 4 | lexopt | 300.9 | 261.3–362.7 | 3,323,750 | 7 |
| 4 | pico-args | 218.1 | 191.0–352.5 | 4,585,894 | 6 |
| 4 | clap_lex | 322.3 | 286.3–429.3 | 3,102,825 | 6 |

## Definition layout

`Command`: 696 bytes. `Arg`: 136 bytes. Default `Parser`: 192 bytes.

The following shallow footprint is `size_of::<Command>() + n * size_of::<Arg>()`. It excludes borrowed name/help strings and external alias/rule slices; it describes this compiler and target.

| Argument definitions | Shallow definition bytes |
|---:|---:|
| 4 | 1240 |
| 8 | 1784 |
| 16 | 2872 |
| 32 | 5048 |
| 64 | 9400 |
| 128 | 18104 |
| 256 | 35512 |

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
