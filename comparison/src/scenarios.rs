//! Additional common grammars. Each adapter consumes the same input and result.

use crate::ParseResult;
use std::ffi::OsString;

const ARGS: [zclap::Arg<'static>; 5] = [
    zclap::Arg::flag("verbose").short('v').repeat(),
    zclap::Arg::flag("quiet").short('q'),
    zclap::Arg::value("port").short('p').required(),
    zclap::Arg::value("tag")
        .short('t')
        .repeat()
        .one_of(&["alpha", "beta", "gamma"]),
    zclap::Arg::positional("input").required(),
];
const LEAF: zclap::Command = zclap::Command::new("bench").args(&ARGS);
const NESTED: zclap::Command = zclap::Command::new("bench")
    .subcommands(&[
        zclap::Command::new("remote").subcommands(&[zclap::Command::new("add").args(&ARGS)])
    ]);

pub const CASES: [(&str, &[&str], bool); 4] = [
    ("short-cluster", &["-vq", "-p", "8080", "input.txt"], false),
    (
        "repeats",
        &[
            "-vvv",
            "--verbose",
            "--tag=alpha",
            "--tag",
            "beta",
            "--tag=gamma",
            "--port=8080",
            "input.txt",
        ],
        false,
    ),
    (
        "mixed",
        &["-qv", "-p8080", "--tag=alpha", "--tag", "beta", "input.txt"],
        false,
    ),
    (
        "subcommands",
        &["remote", "add", "-vq", "-p8080", "--tag=alpha", "input.txt"],
        true,
    ),
];

#[derive(Default)]
struct Output {
    verbose: u64,
    quiet: bool,
    port: Option<u64>,
    tags: u64,
    input: Option<u64>,
    depth: usize,
}

impl Output {
    fn flag(&mut self, name: &str) -> Result<(), &'static str> {
        match name {
            "verbose" => self.verbose += 1,
            "quiet" if !self.quiet => self.quiet = true,
            _ => return Err("unknown or duplicate flag"),
        }
        Ok(())
    }

    fn value(&mut self, name: &str, text: &str) -> Result<(), &'static str> {
        match name {
            "port" => self.port_number(text.parse().map_err(|_| "invalid integer")?)?,
            "tag" => self.tags += tag(text)?,
            "input" => self.input_hash(positional_hash(text)?)?,
            _ => return Err("unknown value"),
        }
        Ok(())
    }

    fn port_number(&mut self, port: u64) -> Result<(), &'static str> {
        if self.port.replace(port).is_some() {
            return Err("duplicate port");
        }
        Ok(())
    }

    fn input_hash(&mut self, value: u64) -> Result<(), &'static str> {
        if self.input.replace(value).is_some() {
            return Err("duplicate input");
        }
        Ok(())
    }

    fn command(&mut self, name: &str) -> Result<(), &'static str> {
        match (self.depth, name) {
            (0, "remote") | (1, "add") => self.depth += 1,
            _ => return Err("unknown subcommand"),
        }
        Ok(())
    }

    fn finish(self, nested: bool) -> ParseResult {
        if self.depth != if nested { 2 } else { 0 } {
            return Err("missing subcommand");
        }
        let port = self.port.ok_or("missing port")?;
        let input = self.input.ok_or("missing input")?;
        Ok(input
            .wrapping_add(port.wrapping_mul(17))
            .wrapping_add(self.verbose.wrapping_mul(31))
            .wrapping_add(u64::from(self.quiet) * 43)
            .wrapping_add(self.tags.wrapping_mul(59))
            .wrapping_add(self.depth as u64 * 71))
    }
}

fn tag(text: &str) -> Result<u64, &'static str> {
    match text {
        "alpha" => Ok(1),
        "beta" => Ok(2),
        "gamma" => Ok(3),
        _ => Err("invalid tag"),
    }
}

fn positional_hash(text: &str) -> Result<u64, &'static str> {
    // The shared workload does not accept option-looking positional values.
    // Escaping with `--` is covered by the crate tests, outside this grammar.
    if text.starts_with('-') && text != "-" {
        return Err("unknown option");
    }
    Ok(text.bytes().fold(0u64, |hash, byte| {
        hash.wrapping_mul(131).wrapping_add(u64::from(byte))
    }))
}

fn clap_leaf(name: &'static str) -> clap::Command {
    clap::Command::new(name)
        .disable_help_flag(true)
        .disable_version_flag(true)
        .arg(
            clap::Arg::new("verbose")
                .long("verbose")
                .short('v')
                .action(clap::ArgAction::Count),
        )
        .arg(
            clap::Arg::new("quiet")
                .long("quiet")
                .short('q')
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            clap::Arg::new("port")
                .long("port")
                .short('p')
                .required(true)
                .value_parser(clap::value_parser!(u64)),
        )
        .arg(
            clap::Arg::new("tag")
                .long("tag")
                .short('t')
                .action(clap::ArgAction::Append)
                .value_parser(["alpha", "beta", "gamma"]),
        )
        .arg(clap::Arg::new("input").required(true))
}

pub struct Fixture {
    nested: bool,
    clap: clap::Command,
}

impl Fixture {
    pub fn new(nested: bool) -> Self {
        let mut clap = if nested {
            clap::Command::new("bench")
                .disable_help_flag(true)
                .disable_help_subcommand(true)
                .disable_version_flag(true)
                .subcommand_required(true)
                .subcommand(
                    clap::Command::new("remote")
                        .disable_help_flag(true)
                        .disable_help_subcommand(true)
                        .disable_version_flag(true)
                        .subcommand_required(true)
                        .subcommand(clap_leaf("add")),
                )
        } else {
            clap_leaf("bench")
        }
        .no_binary_name(true);
        clap.build();
        Self { nested, clap }
    }

    pub fn run(&mut self, engine: usize, input: &[&str]) -> ParseResult {
        match engine {
            0 => self.zclap(input),
            1 => self.clap(input),
            2 => self.lexopt(input),
            3 => self.pico(input),
            4 => self.clap_lex(input),
            _ => unreachable!(),
        }
    }

    fn zclap(&self, input: &[&str]) -> ParseResult {
        let mut parser = if self.nested { &NESTED } else { &LEAF }.parse(input);
        let mut output = Output::default();
        while let Some(event) = parser.next().map_err(|_| "parse error")? {
            match event {
                zclap::Event::Flag { arg } => output.flag(arg)?,
                zclap::Event::Value { arg, value } => output.value(arg, value)?,
                zclap::Event::Command { name } => output.command(name)?,
                _ => return Err("unexpected event"),
            }
        }
        parser.finish().map_err(|_| "validation error")?;
        output.finish(self.nested)
    }

    fn clap(&mut self, input: &[&str]) -> ParseResult {
        let matches = self
            .clap
            .try_get_matches_from_mut(input.iter().copied())
            .map_err(|_| "parse error")?;
        let mut output = Output::default();
        let leaf = if self.nested {
            output.command("remote")?;
            output.command("add")?;
            matches
                .subcommand_matches("remote")
                .and_then(|remote| remote.subcommand_matches("add"))
                .ok_or("missing subcommand")?
        } else {
            &matches
        };
        output.verbose = u64::from(leaf.get_count("verbose"));
        output.quiet = leaf.get_flag("quiet");
        output.port_number(*leaf.get_one::<u64>("port").ok_or("missing port")?)?;
        if let Some(tags) = leaf.get_many::<String>("tag") {
            for value in tags {
                output.value("tag", value)?;
            }
        }
        output.value(
            "input",
            leaf.get_one::<String>("input").ok_or("missing input")?,
        )?;
        output.finish(self.nested)
    }

    fn lexopt(&self, input: &[&str]) -> ParseResult {
        let mut parser = lexopt::Parser::from_args(input.iter().copied());
        let mut output = Output::default();
        while let Some(token) = parser.next().map_err(|_| "parse error")? {
            if self.nested && output.depth < 2 {
                let lexopt::Arg::Value(name) = token else {
                    return Err("expected subcommand");
                };
                output.command(name.to_str().ok_or("invalid subcommand")?)?;
                continue;
            }
            match token {
                lexopt::Arg::Short('v') | lexopt::Arg::Long("verbose") => output.flag("verbose")?,
                lexopt::Arg::Short('q') | lexopt::Arg::Long("quiet") => output.flag("quiet")?,
                lexopt::Arg::Short('p') | lexopt::Arg::Long("port") => {
                    let value = parser.value().map_err(|_| "missing value")?;
                    output.value("port", value.to_str().ok_or("invalid port")?)?;
                }
                lexopt::Arg::Short('t') | lexopt::Arg::Long("tag") => {
                    let value = parser.value().map_err(|_| "missing value")?;
                    output.value("tag", value.to_str().ok_or("invalid tag")?)?;
                }
                lexopt::Arg::Value(value) => {
                    output.value("input", value.to_str().ok_or("invalid input")?)?;
                }
                _ => return Err("unknown option"),
            }
        }
        output.finish(self.nested)
    }

    fn pico(&self, input: &[&str]) -> ParseResult {
        let mut parser = pico_args::Arguments::from_vec(input.iter().map(OsString::from).collect());
        let mut output = Output::default();
        if self.nested {
            for _ in 0..2 {
                let name = parser
                    .subcommand()
                    .map_err(|_| "invalid subcommand")?
                    .ok_or("missing subcommand")?;
                output.command(&name)?;
            }
        }
        // pico-args requires value queries before combined flag queries.
        let port: u64 = parser
            .value_from_str(["-p", "--port"])
            .map_err(|_| "invalid port")?;
        output.port_number(port)?;
        while let Some(value) = parser
            .opt_value_from_fn(["-t", "--tag"], tag)
            .map_err(|_| "invalid tag")?
        {
            output.tags += value;
        }
        while parser.contains(["-v", "--verbose"]) {
            output.flag("verbose")?;
        }
        if parser.contains(["-q", "--quiet"]) {
            output.flag("quiet")?;
        }
        output.input_hash(
            parser
                .free_from_fn(positional_hash)
                .map_err(|_| "invalid input")?,
        )?;
        if !parser.finish().is_empty() {
            return Err("unknown or duplicate argument");
        }
        output.finish(self.nested)
    }

    fn clap_lex(&self, input: &[&str]) -> ParseResult {
        let raw = clap_lex::RawArgs::new(input.iter().copied());
        let mut cursor = raw.cursor();
        let mut output = Output::default();
        while let Some(token) = raw.next(&mut cursor) {
            if self.nested && output.depth < 2 {
                output.command(token.to_value_os().to_str().ok_or("invalid subcommand")?)?;
                continue;
            }
            if let Some((name, attached)) = token.to_long() {
                match name.map_err(|_| "invalid option")? {
                    name @ ("verbose" | "quiet") => {
                        if attached.is_some() {
                            return Err("flag has a value");
                        }
                        output.flag(name)?;
                    }
                    name @ ("port" | "tag") => {
                        let value = attached
                            .or_else(|| raw.next_os(&mut cursor))
                            .ok_or("missing value")?;
                        output.value(name, value.to_str().ok_or("invalid value")?)?;
                    }
                    _ => return Err("unknown option"),
                }
            } else if let Some(mut short) = token.to_short() {
                while let Some(name) = short.next_flag() {
                    match name.map_err(|_| "invalid short option")? {
                        'v' => output.flag("verbose")?,
                        'q' => output.flag("quiet")?,
                        name @ ('p' | 't') => {
                            let value = short
                                .next_value_os()
                                .or_else(|| raw.next_os(&mut cursor))
                                .ok_or("missing value")?;
                            output.value(
                                if name == 'p' { "port" } else { "tag" },
                                value.to_str().ok_or("invalid value")?,
                            )?;
                        }
                        _ => return Err("unknown short option"),
                    }
                }
            } else {
                output.value(
                    "input",
                    token.to_value_os().to_str().ok_or("invalid input")?,
                )?;
            }
        }
        output.finish(self.nested)
    }

    pub fn verify(&mut self, input: &[&str]) {
        let expected = self.run(0, input).expect("valid feature input");
        let invalid: &[&[&str]] = &[
            &[],
            &["--port=8080"],
            &["input.txt"],
            &["--port"],
            &["--port=1", "--port=2", "input.txt"],
            &["-qq", "--port=1", "input.txt"],
            &["--port=1", "--unknown", "input.txt"],
            &["--port=bad", "input.txt"],
            &["--port=1", "--tag=invalid", "input.txt"],
            &["--port=1", "--tag", "--quiet", "input.txt"],
            &["--port=1", "input.txt", "extra.txt"],
            &["--verbose=yes", "--port=1", "input.txt"],
        ];
        for (engine, name) in crate::ENGINES.iter().enumerate() {
            assert_eq!(
                self.run(engine, input),
                Ok(expected),
                "{name}: feature checksum mismatch"
            );
            for &case in invalid {
                let nested;
                let case = if self.nested {
                    nested = ["remote", "add"]
                        .into_iter()
                        .chain(case.iter().copied())
                        .collect::<Vec<_>>();
                    nested.as_slice()
                } else {
                    case
                };
                assert!(
                    self.run(engine, case).is_err(),
                    "{name} accepted invalid feature input: {case:?}"
                );
            }
            if self.nested {
                for case in [
                    &[][..],
                    &["remote"][..],
                    &["remote", "unknown"][..],
                    &["unknown", "add"][..],
                ] {
                    assert!(
                        self.run(engine, case).is_err(),
                        "{name} accepted invalid subcommands: {case:?}"
                    );
                }
            }
        }
    }
}
