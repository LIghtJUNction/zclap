# zclap

Small, allocation-free CLI parsing. Six public building blocks: `Command`,
`Arg`, `Parser`, `Event`, `Error`, and `Style`. Definitions can be constants;
events borrow the input. No third-party dependencies, including test/build
dependencies. The crate is always `#![no_std]`; optional `std` adds OS strings.

```toml
[dependencies]
zclap = "0.1.0"
```

```rust
use zclap::{Arg, Command, Event};

const CLI: Command = Command::new("demo").about("Minimal CLI").args(&[
    Arg::flag("verbose").short('v').repeat(),
    Arg::value("port").short('p'),
    Arg::positional("input").required(),
]);

let argv = ["-vv", "--port=8080", "input.txt"];
let mut parser = CLI.parse(&argv);
while let Some(event) = parser.next()? {
    match event {
        Event::Flag { arg: "verbose" } => {},
        Event::Value { arg, value } => { /* process borrowed &str */ },
        _ => {},
    }
}
parser.finish()?;
# Ok::<(), zclap::Error<'static>>(())
```

## Grammar and validation

Pass tokens **without the executable name**. Named arguments default to
`--<id>`; `.long()` changes the spelling. Supports `-v`, `--verbose`, `-vvv`,
`--port 8080`, `--port=8080`, `-p8080`, `-p=8080`, and positional values.
`--` ends option and subcommand recognition; `-` is a positional value.
Empty attached values are valid. A value starting with `-` must be attached
to its option. No whitespace splitting, quote interpretation or shell expansion
is performed: the shell/OS has already separated arguments.

`.required()`, `.repeat()`, `.alias()`, `.one_of()`, `.conflicts()` and
`.requires()` compose. Relationships refer to IDs in the same command. Each
builder setting replaces its previous value; aliases and relationship rules
each hold one target. `.values(min, max)` consumes a range of values, emitting
one event per value. A new option ends a value group; a subcommand ends it
once its minimum has been met. A variable positional must be the last positional.
Required positionals precede optional ones.

```rust
use zclap::{Arg, Command};
const CLI: Command = Command::new("tool").subcommands(&[
    Command::new("build").args(&[
        Arg::value("mode").one_of(&["debug", "release"]),
        Arg::value("include").short('I').values(1, usize::MAX).repeat(),
        Arg::positional("source").required(),
    ]),
]);
let argv = ["build", "--mode=release", "main.rs"];
let mut parser = CLI.parse(&argv);
while let Some(event) = parser.next()? { /* handle Event::Command and values */ }
parser.finish()?;
# Ok::<(), zclap::Error<'static>>(())
```

Parent arguments precede the subcommand; entering a child validates the parent.
Children have independent scopes and may nest without a fixed depth limit.
Consume all events and call `finish()` for final validation. Errors are terminal
and repeatable, carry IDs/indices, and provide exit code 2 without exiting.
Help/version flags and early success exits are controlled by the application.

## Styling

Rendering never participates in parsing. Any `core::fmt::Write` sink works,
including fixed buffers. `Style::plain()` emits no colors; `Style::default()`
uses ANSI. The application chooses when colors are appropriate.

```rust
use zclap::{Command, Style};
let mut style = Style::plain();
style.title = ("\x1b[1;36m", "\x1b[0m");
style.indent = 4;
style.gap = 3;
style.width = 72;
let mut output = String::new();
style.help(&Command::new("demo"), &mut output)?;
# Ok::<(), core::fmt::Error>(())
```

Title, option, value, error and hint spans have independent prefix/suffix pairs.
Replace `help_layout` or `error_layout` for a completely custom layout, or use
definition accessors with your own formatter. See `examples/style.rs`.
Wrapping counts Unicode scalar values, not terminal display cells.

## no_std, OS strings and capacity

```toml
zclap = { version = "0.1.0", default-features = false }
```

`parse(&[&str])` borrows text. `parse_bytes(&[&[u8]])` preserves arbitrary bytes.
With `std`, `parse_os(&[OsString])` borrows native `OsStr` values, preserving
Unix non-UTF-8 filenames and Windows unpaired UTF-16 surrogates. `parse()` also
accepts `String` and `&OsStr` inputs. Collecting `std::env::args_os().skip(1)` is
the caller's allocation; the parser adds none. The single audited unsafe block
in the OS adapter reconstructs borrowed suffixes only at validated UTF-8 option
boundaries. The parsing core contains no unsafe code.

The default parser stores 256 presence bits. For more definitions, use
`command.parse_with::<8, _>(&argv)` for 512 slots (64 per word); input and repeat
counts have no fixed limit. A capacity shortage returns an error. Definitions
are checked when built; `command.check()` exposes errors independently.

MSRV: **Rust 1.74.0**. Stable Linux, macOS and Windows, default/minimal features,
and a bare `thumbv7em-none-eabi` no_std target are covered by CI.

## Performance and releases

The isolated [`comparison/`](comparison/) project compares the same grammar and
inputs with clap, lexopt, pico-args and clap_lex at 4–256 definitions. It records
latency, throughput, allocation requests, binary size and clean compile time.
On the recorded Linux / Xeon 8573C host (Rust 1.99.1, opt-level 3, thin LTO):

| Dense input | zclap | clap | lexopt | pico-args | clap_lex |
|---|---:|---:|---:|---:|---:|
| 4 definitions | 0.159 µs | 2.263 µs | 0.252 µs | 0.195 µs | 0.283 µs |
| 256 definitions | 19.379 µs | 14,496.541 µs | 25.194 µs | 159.931 µs | 26.023 µs |

zclap allocated zero times in all measured cases. Its 256-option dense median
fell from 119.3 µs to 19.4 µs after indexing long names and required-state checks.
These are complete parses of the shared long-option grammar, including result
consumption. Shared-host scheduling produces outliers; see the
[full report and every sample](comparison/results/final.md) for ranges, all
sizes, allocations, binary sizes and compilation times. Reproduce with
`python3 comparison/run.py --label local`. Results describe these workloads,
not a universal speed guarantee.

`Release` → `Run workflow` supports patch/minor/major/manual versions, `current`
for the first 0.1.0 release, and `resume` for an interrupted publication. Add
the repository Actions secret `CARGO_REGISTRY_TOKEN`; a local variable is not
inherited by GitHub. Cross-platform checks gate the version commit/tag,
crates.io publication and GitHub Release. Missing credentials stop before
modifying the repository.

To request a release through Git, commit a change to
`.github/release-request.json` on `main`, containing exactly
`{"bump":"current","version":""}` for the first release, or the same supported
bump/version choices as the manual workflow. Changing this file intentionally
starts the same credential checks, full CI, and publication pipeline. Other
pushes do not trigger publication; the default-branch guard still applies.

MIT license.
