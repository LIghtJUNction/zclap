use std::ffi::OsStr;
use zclap::{Arg, Command, Error, Event, Style};

const CLI: Command = Command::new("demo")
    .about("Borrowed command-line events")
    .args(&[
        Arg::flag("help").short('h').help("Show this help"),
        Arg::flag("verbose")
            .short('v')
            .repeat()
            .help("Increase verbosity"),
        Arg::value("port").short('p').help("Listen on this port"),
        Arg::positional("input").help("Input file"),
    ])
    .subcommands(&[Command::new("check").about("Check input files").args(&[
        Arg::flag("help").short('h').help("Show command help"),
        Arg::positional("files")
            .required()
            .repeat()
            .help("Files to check"),
    ])]);

fn run() -> Result<(), Error<'static>> {
    let argv: Vec<_> = std::env::args_os().skip(1).collect();
    let mut parser = CLI.parse_os(&argv);
    while let Some(event) = parser.next()? {
        match event {
            Event::Flag { arg: "help" } => {
                // Formatting to String is this example's choice; embedded users
                // can supply a fixed-capacity core::fmt::Write sink instead.
                let mut help = String::new();
                Style::plain().help(parser.command(), &mut help).unwrap();
                print!("{help}");
                return Ok(());
            }
            Event::Value { arg: "port", value } if value == OsStr::new("8080") => {
                println!("listening on 8080");
            }
            event => println!("{event:?}"),
        }
    }
    parser.finish()
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(error.exit_code());
    }
}
