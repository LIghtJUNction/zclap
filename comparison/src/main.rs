use std::collections::HashMap;
use std::ffi::OsString;
use std::hint::black_box;
#[cfg(not(feature = "allocation-counting"))]
use std::time::{Duration, Instant};
use zclap_comparison::{contribution, contribution_number, name_index, Output};

mod scenarios;

#[cfg(feature = "allocation-counting")]
mod allocations {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

    pub struct Counting;
    pub static ENABLED: AtomicBool = AtomicBool::new(false);
    pub static CALLS: AtomicU64 = AtomicU64::new(0);
    pub static BYTES: AtomicU64 = AtomicU64::new(0);

    // This allocator is confined to the unpublished benchmark executable.
    // Every pointer/layout is forwarded unchanged to Rust's System allocator.
    unsafe impl GlobalAlloc for Counting {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            if ENABLED.load(Ordering::Relaxed) {
                CALLS.fetch_add(1, Ordering::Relaxed);
                BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
            }
            unsafe { System.alloc(layout) }
        }
        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            unsafe { System.dealloc(ptr, layout) }
        }
        unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
            if ENABLED.load(Ordering::Relaxed) {
                CALLS.fetch_add(1, Ordering::Relaxed);
                BYTES.fetch_add(new_size as u64, Ordering::Relaxed);
            }
            unsafe { System.realloc(ptr, layout, new_size) }
        }
        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            if ENABLED.load(Ordering::Relaxed) {
                CALLS.fetch_add(1, Ordering::Relaxed);
                BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
            }
            unsafe { System.alloc_zeroed(layout) }
        }
    }
}

#[cfg(feature = "allocation-counting")]
#[global_allocator]
static ALLOCATOR: allocations::Counting = allocations::Counting;

const ENGINES: [&str; 5] = ["zclap", "clap", "lexopt", "pico-args", "clap_lex"];
const SIZES: [usize; 7] = [4, 8, 16, 32, 64, 128, 256];

type ParseResult = Result<u64, &'static str>;

struct Fixture {
    names: Vec<&'static str>,
    keys: Vec<&'static str>,
    lookup: HashMap<&'static str, usize>,
    zclap: zclap::Command<'static>,
    clap: clap::Command,
}

impl Fixture {
    fn new(count: usize) -> Self {
        Self::named(count, "bench")
    }

    fn named(count: usize, command_name: &'static str) -> Self {
        let names: Vec<&'static str> = (0..count)
            .map(|i| &*Box::leak(format!("opt{i:03}").into_boxed_str()))
            .collect();
        let keys = names
            .iter()
            .map(|name| &*Box::leak(format!("--{name}").into_boxed_str()))
            .collect();
        let lookup = names
            .iter()
            .enumerate()
            .map(|(i, &name)| (name, i))
            .collect();
        let args: Vec<_> = names
            .iter()
            .enumerate()
            .map(|(i, &name)| {
                let arg = if i % 2 == 0 {
                    zclap::Arg::value(name)
                } else {
                    zclap::Arg::flag(name)
                };
                if i == 0 {
                    arg.required()
                } else {
                    arg
                }
            })
            .collect();
        let zclap = zclap::Command::new(command_name).args(Box::leak(args.into_boxed_slice()));
        let mut clap = clap::Command::new(command_name)
            .no_binary_name(true)
            .disable_help_flag(true)
            .disable_version_flag(true);
        for (i, &name) in names.iter().enumerate() {
            let arg = clap::Arg::new(name).long(name).required(i == 0);
            clap = clap.arg(if i % 2 == 0 {
                arg.action(clap::ArgAction::Set)
                    .num_args(1)
                    .value_parser(clap::value_parser!(u64))
            } else {
                arg.action(clap::ArgAction::SetTrue)
            });
        }
        clap.build();
        Self {
            names,
            keys,
            lookup,
            zclap,
            clap,
        }
    }

    fn input(&self, dense: bool) -> Vec<String> {
        let n = self.names.len();
        let mut selected: Vec<usize> = if dense {
            (0..n).collect()
        } else {
            vec![0, n / 3, n / 2, n - 1]
        };
        selected.sort_unstable();
        selected.dedup();
        // A fixed permutation avoids advantaging definition-order consumption.
        if selected.len() > 2 {
            let last = selected.len() - 1;
            selected.swap(1, last);
        }
        let mut input = Vec::new();
        for i in selected {
            if i % 2 == 0 {
                if i % 4 == 0 {
                    input.push(format!("{}={}", self.keys[i], 1000 + i));
                } else {
                    input.push(self.keys[i].to_owned());
                    input.push((1000 + i).to_string());
                }
            } else {
                input.push(self.keys[i].to_owned());
            }
        }
        input
    }

    fn run(&mut self, engine: usize, input: &[&str]) -> ParseResult {
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
        let mut parser = self.zclap.parse(input);
        let mut checksum = 0u64;
        while let Some(event) = parser.next().map_err(|_| "parse error")? {
            let (arg, value) = match event {
                zclap::Event::Flag { arg } => (arg, None),
                zclap::Event::Value { arg, value } => (arg, Some(value)),
                _ => return Err("unexpected command"),
            };
            checksum = checksum.wrapping_add(contribution(name_index(arg)?, value)?);
        }
        parser.finish().map_err(|_| "validation error")?;
        Ok(checksum)
    }

    fn clap(&mut self, input: &[&str]) -> ParseResult {
        let matches = self
            .clap
            .try_get_matches_from_mut(input.iter().copied())
            .map_err(|_| "parse error")?;
        let mut checksum = 0u64;
        for id in matches.ids() {
            if matches.value_source(id.as_str()) != Some(clap::parser::ValueSource::CommandLine) {
                continue;
            }
            let index = name_index(id.as_str())?;
            let numeric = if index % 2 == 0 {
                *matches.get_one::<u64>(id.as_str()).ok_or("missing value")?
            } else {
                1
            };
            checksum = checksum.wrapping_add(contribution_number(index, numeric));
        }
        Ok(checksum)
    }

    fn lexopt(&self, input: &[&str]) -> ParseResult {
        let mut parser = lexopt::Parser::from_args(input.iter().copied());
        let mut output = Output::default();
        while let Some(token) = parser.next().map_err(|_| "parse error")? {
            let index = match token {
                lexopt::Arg::Long(name) => *self.lookup.get(name).ok_or("unknown option")?,
                _ => return Err("unexpected token"),
            };
            if index % 2 == 0 {
                let value = parser.value().map_err(|_| "missing value")?;
                let text = value.to_str().ok_or("non-UTF-8 integer")?;
                output.add(index, Some(text))?;
            } else {
                output.add(index, None)?;
            }
        }
        output.finish()
    }

    fn pico(&self, input: &[&str]) -> ParseResult {
        let mut parser = pico_args::Arguments::from_vec(input.iter().map(OsString::from).collect());
        let mut output = Output::default();
        // Query values before flags, as pico-args requires with combined-flags.
        for i in (0..self.names.len()).step_by(2) {
            let value: Option<u64> = parser
                .opt_value_from_str(self.keys[i])
                .map_err(|_| "value error")?;
            if let Some(value) = value {
                output.add_number(i, value)?;
            }
        }
        for i in (1..self.names.len()).step_by(2) {
            if parser.contains(self.keys[i]) {
                output.add(i, None)?;
            }
        }
        if !parser.finish().is_empty() {
            return Err("unknown or duplicate option");
        }
        output.finish()
    }

    fn clap_lex(&self, input: &[&str]) -> ParseResult {
        let raw = clap_lex::RawArgs::new(input.iter().copied());
        let mut cursor = raw.cursor();
        let mut output = Output::default();
        while let Some(token) = raw.next(&mut cursor) {
            let (name, attached) = token.to_long().ok_or("unexpected token")?;
            let index = *self
                .lookup
                .get(name.map_err(|_| "non-UTF-8 option")?)
                .ok_or("unknown option")?;
            if index % 2 == 0 {
                let value = attached
                    .or_else(|| raw.next_os(&mut cursor))
                    .ok_or("missing value")?;
                output.add(index, Some(value.to_str().ok_or("non-UTF-8 integer")?))?;
            } else {
                if attached.is_some() {
                    return Err("flag has a value");
                }
                output.add(index, None)?;
            }
        }
        output.finish()
    }
}

fn verify(fixture: &mut Fixture, input: &[&str]) -> u64 {
    let expected = fixture.run(0, input).expect("zclap valid input");
    for (engine, name) in ENGINES.iter().enumerate() {
        assert_eq!(
            fixture.run(engine, input),
            Ok(expected),
            "semantic mismatch: {name}"
        );
        for invalid in [
            vec![],
            vec!["--unknown"],
            vec!["--opt000"],
            vec!["--opt000=1", "--opt000=2"],
            vec!["--opt000=1", "--opt001", "--opt001"],
            vec!["--opt000=1", "--opt001=yes"],
            vec!["--opt000=bad"],
            vec!["--opt000", "--opt001"],
            vec!["--opt000=1", "extra"],
        ] {
            assert!(
                fixture.run(engine, &invalid).is_err(),
                "{name} accepted {invalid:?}"
            );
        }
    }
    expected
}

#[cfg(not(feature = "allocation-counting"))]
fn measure(mut run: impl FnMut() -> ParseResult, iterations: usize) -> f64 {
    let start = Instant::now();
    for _ in 0..iterations {
        black_box(run().unwrap());
    }
    start.elapsed().as_nanos() as f64 / iterations as f64
}

fn record(
    scenario: &str,
    count: usize,
    rounds: usize,
    millis: u64,
    mut run: impl FnMut(usize) -> ParseResult,
) {
    #[cfg(not(feature = "allocation-counting"))]
    {
        let mut iterations = [1usize; 5];
        for (engine, item) in iterations.iter_mut().enumerate() {
            // Warm parser caches and calibrate a duration per parser.
            let mut batch = 32usize;
            loop {
                let elapsed = measure(|| run(engine), batch) * batch as f64;
                if elapsed >= Duration::from_millis(3).as_nanos() as f64 || batch >= 1 << 22 {
                    *item =
                        ((millis as f64 * 1_000_000.0 / elapsed) * batch as f64).max(32.0) as usize;
                    break;
                }
                batch *= 2;
            }
        }
        for round in 0..rounds {
            for shift in 0..ENGINES.len() {
                let engine = (shift + round) % ENGINES.len();
                let ns = measure(|| run(engine), iterations[engine]);
                println!(
                    "{scenario},{count},{},{round},{},{ns:.3},{:.3}",
                    ENGINES[engine],
                    iterations[engine],
                    1e9 / ns
                );
            }
        }
    }
    #[cfg(feature = "allocation-counting")]
    {
        use std::sync::atomic::Ordering;
        let _ = (rounds, millis);
        for (engine, name) in ENGINES.iter().enumerate() {
            black_box(run(engine).unwrap());
            allocations::CALLS.store(0, Ordering::Relaxed);
            allocations::BYTES.store(0, Ordering::Relaxed);
            allocations::ENABLED.store(true, Ordering::Relaxed);
            black_box(run(engine).unwrap());
            allocations::ENABLED.store(false, Ordering::Relaxed);
            let calls = allocations::CALLS.load(Ordering::Relaxed);
            let bytes = allocations::BYTES.load(Ordering::Relaxed);
            println!("{scenario},{count},{name},{calls},{bytes}");
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).is_some_and(|v| v == "--layout") {
        println!(
            "{{\"command_bytes\":{},\"arg_bytes\":{},\"parser_bytes\":{}}}",
            std::mem::size_of::<zclap::Command>(),
            std::mem::size_of::<zclap::Arg>(),
            std::mem::size_of::<zclap::Parser>(),
        );
        return;
    }
    let checks_only = args.get(1).is_some_and(|v| v == "--check");
    let rounds: usize = if checks_only {
        3
    } else {
        args.get(1).map_or(7, |v| v.parse().expect("round count"))
    };
    let millis: u64 = args
        .get(2)
        .map_or(40, |v| v.parse().expect("milliseconds per sample"));
    assert!(rounds >= 3 && millis > 0);
    if !checks_only {
        #[cfg(not(feature = "allocation-counting"))]
        println!("scenario,options,parser,round,iterations,ns_per_parse,parses_per_second");
        #[cfg(feature = "allocation-counting")]
        println!("scenario,options,parser,allocations,allocated_bytes");
    }
    for count in SIZES {
        let mut fixture = Fixture::new(count);
        for (scenario, dense) in [("sparse", false), ("dense", true)] {
            let owned = fixture.input(dense);
            let input: Vec<_> = owned.iter().map(String::as_str).collect();
            black_box(verify(&mut fixture, &input));
            if !checks_only {
                record(scenario, count, rounds, millis, |engine| {
                    fixture.run(black_box(engine), black_box(&input))
                });
            }
        }
    }
    for (name, input, nested) in scenarios::CASES {
        let mut fixture = scenarios::Fixture::new(nested);
        fixture.verify(input);
        if !checks_only {
            record(name, 5, rounds, millis, |engine| {
                fixture.run(black_box(engine), black_box(input))
            });
        }
    }
    for length in [8, 32, 128] {
        let command_name = Box::leak("c".repeat(length).into_boxed_str());
        let mut fixture = Fixture::named(4, command_name);
        let owned = fixture.input(true);
        let input: Vec<_> = owned.iter().map(String::as_str).collect();
        black_box(verify(&mut fixture, &input));
        if !checks_only {
            record(
                &format!("command-name-{length}"),
                4,
                rounds,
                millis,
                |engine| fixture.run(black_box(engine), black_box(&input)),
            );
        }
    }
    if checks_only {
        println!("All shared-grammar checks passed.");
    }
}
