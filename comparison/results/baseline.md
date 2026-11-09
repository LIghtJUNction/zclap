# Recorded comparison

Generated: 2026-11-09T18:34:21.007245+00:00

CPU: INTEL(R) XEON(R) PLATINUM 8573C. OS: Linux-6.18.57-x86_64-with-glibc2.41.
Rust: `rustc 1.99.1 (58cf8f9c3 2026-10-13)`. Pinned CPU: 0.

Release profile: opt-level 3, thin LTO, one codegen unit, stripped executables.

The JSON and CSV files contain every sample. A range is the observed minimum–maximum, not a confidence interval.

## Sparse long options

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 4 | zclap | 200.2 | 191.1–217.4 | 4,995,779 | 0 |
| 4 | clap | 3382.5 | 2851.3–3498.9 | 295,636 | 51 |
| 4 | lexopt | 439.3 | 353.9–473.8 | 2,276,504 | 7 |
| 4 | pico-args | 266.4 | 237.1–275.9 | 3,754,303 | 6 |
| 4 | clap_lex | 421.6 | 379.8–487.9 | 2,371,787 | 6 |
| 8 | zclap | 294.5 | 249.7–311.2 | 3,395,678 | 0 |
| 8 | clap | 5434.0 | 5317.2–5660.3 | 184,028 | 77 |
| 8 | lexopt | 421.2 | 329.9–554.9 | 2,373,893 | 8 |
| 8 | pico-args | 424.1 | 395.0–426.0 | 2,358,151 | 6 |
| 8 | clap_lex | 447.7 | 438.5–456.1 | 2,233,644 | 6 |
| 16 | zclap | 275.0 | 258.1–378.1 | 3,636,483 | 0 |
| 16 | clap | 8333.2 | 6850.4–9987.3 | 120,002 | 102 |
| 16 | lexopt | 370.9 | 350.3–423.0 | 2,696,195 | 7 |
| 16 | pico-args | 769.1 | 682.9–1053.1 | 1,300,251 | 5 |
| 16 | clap_lex | 382.1 | 367.1–499.8 | 2,616,774 | 5 |
| 32 | zclap | 390.3 | 307.6–457.3 | 2,561,954 | 0 |
| 32 | clap | 16919.3 | 11965.2–18035.5 | 59,104 | 177 |
| 32 | lexopt | 298.9 | 240.7–635.8 | 3,345,903 | 8 |
| 32 | pico-args | 1330.8 | 1016.9–1742.2 | 751,416 | 6 |
| 32 | clap_lex | 404.8 | 269.2–555.1 | 2,470,405 | 6 |
| 64 | zclap | 802.1 | 604.0–1699.3 | 1,246,793 | 0 |
| 64 | clap | 56142.3 | 34151.6–237792.9 | 17,812 | 298 |
| 64 | lexopt | 374.5 | 215.7–521.7 | 2,669,906 | 7 |
| 64 | pico-args | 2671.9 | 2173.6–2912.9 | 374,270 | 5 |
| 64 | clap_lex | 433.0 | 381.4–572.4 | 2,309,394 | 5 |
| 128 | zclap | 1468.5 | 1268.2–1829.9 | 680,958 | 0 |
| 128 | clap | 104653.1 | 93375.0–166487.5 | 9,555 | 565 |
| 128 | lexopt | 347.1 | 260.0–426.2 | 2,880,856 | 8 |
| 128 | pico-args | 4218.0 | 3747.1–8705.0 | 237,078 | 6 |
| 128 | clap_lex | 336.0 | 281.4–418.7 | 2,975,960 | 6 |
| 256 | zclap | 3040.6 | 2126.4–8161.3 | 328,881 | 0 |
| 256 | clap | 527945.8 | 285206.8–2211058.2 | 1,894 | 1070 |
| 256 | lexopt | 776.8 | 340.0–1181.9 | 1,287,374 | 7 |
| 256 | pico-args | 9842.4 | 9159.3–26251.0 | 101,601 | 5 |
| 256 | clap_lex | 333.4 | 285.9–1613.0 | 2,999,103 | 5 |

## Dense long options

| Definitions | Parser | Median ns/parse ↓ | Min–max ns | Parses/s ↑ | Allocations/parse ↓ |
|---:|---|---:|---:|---:|---:|
| 4 | zclap | 209.0 | 198.7–280.4 | 4,783,842 | 0 |
| 4 | clap | 2981.3 | 2736.7–3791.6 | 335,420 | 51 |
| 4 | lexopt | 378.3 | 364.6–504.0 | 2,643,293 | 7 |
| 4 | pico-args | 285.9 | 245.0–566.4 | 3,497,592 | 6 |
| 4 | clap_lex | 385.7 | 357.9–610.5 | 2,592,574 | 6 |
| 8 | zclap | 486.2 | 418.6–553.2 | 2,056,822 | 0 |
| 8 | clap | 7881.3 | 6827.9–9637.8 | 126,882 | 100 |
| 8 | lexopt | 776.8 | 721.0–1061.3 | 1,287,283 | 13 |
| 8 | pico-args | 715.6 | 638.0–1363.2 | 1,397,521 | 11 |
| 8 | clap_lex | 898.9 | 823.0–1033.6 | 1,112,432 | 11 |
| 16 | zclap | 1157.6 | 1006.3–1392.1 | 863,884 | 0 |
| 16 | clap | 22647.7 | 21477.4–26849.1 | 44,155 | 194 |
| 16 | lexopt | 1252.6 | 899.3–1568.0 | 798,370 | 25 |
| 16 | pico-args | 1467.5 | 1015.2–1973.3 | 681,441 | 21 |
| 16 | clap_lex | 1464.5 | 1042.8–1782.8 | 682,828 | 21 |
| 32 | zclap | 5314.7 | 4235.0–14057.0 | 188,156 | 0 |
| 32 | clap | 180599.1 | 93155.1–300063.7 | 5,537 | 378 |
| 32 | lexopt | 3065.0 | 1763.7–5092.1 | 326,262 | 49 |
| 32 | pico-args | 5639.9 | 2870.9–15722.5 | 177,309 | 41 |
| 32 | clap_lex | 5741.7 | 3358.2–8694.3 | 174,164 | 41 |
| 64 | zclap | 9793.8 | 8195.2–11872.5 | 102,105 | 0 |
| 64 | clap | 641903.6 | 309348.0–1757871.9 | 1,558 | 742 |
| 64 | lexopt | 8975.7 | 5815.9–16550.0 | 111,412 | 97 |
| 64 | pico-args | 13899.4 | 11555.7–75487.7 | 71,946 | 81 |
| 64 | clap_lex | 9823.6 | 6387.2–11907.7 | 101,796 | 81 |
| 128 | zclap | 63459.1 | 32758.1–123539.1 | 15,758 | 0 |
| 128 | clap | 2519662.4 | 1682202.8–9461223.2 | 397 | 1466 |
| 128 | lexopt | 20251.4 | 16614.6–130199.0 | 49,379 | 193 |
| 128 | pico-args | 46827.3 | 38366.6–105138.2 | 21,355 | 161 |
| 128 | clap_lex | 16100.7 | 10404.6–248557.4 | 62,109 | 161 |
| 256 | zclap | 127307.2 | 96961.4–229455.8 | 7,855 | 0 |
| 256 | clap | 15603126.0 | 9867645.0–156321166.0 | 64 | 2910 |
| 256 | lexopt | 37979.0 | 21890.4–65093.1 | 26,330 | 385 |
| 256 | pico-args | 192054.8 | 150608.5–222076.1 | 5,207 | 321 |
| 256 | clap_lex | 32547.2 | 24320.5–89548.5 | 30,725 | 321 |

## Scope

- Shared cloud host; CPU affinity is used where supported; turbo and neighboring workloads are not controlled.
- Timing builds have no counting allocator. Allocation builds count alloc, alloc_zeroed and realloc calls separately from timing.
- Definitions are prepared once outside the timed loop. Every parse includes input ownership required by each API, semantic validation, result reading and drop.
- Build timings start with an empty target directory and use a warm local registry cache; network download time is excluded.

The timed workloads use valid UTF-8 long options. They do not measure help rendering, error formatting, short clusters, subcommands, or operating-system process startup.
Small-parser adapters perform the semantic checks absent from their lexer APIs. The harness rejects unknown, repeated, missing, and invalid numeric values before collecting samples.
