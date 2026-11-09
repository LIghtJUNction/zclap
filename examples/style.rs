use core::fmt::{self, Write};
use zclap::{Arg, Command, Style};

const CLI: Command = Command::new("paint")
    .about("Configure help independently from argument parsing.")
    .args(&[
        Arg::flag("verbose").short('v').help("Show progress"),
        Arg::value("color")
            .one_of(&["red", "blue"])
            .help("Output color"),
        Arg::positional("file").required().help("Input file"),
    ]);

fn compact(command: &Command<'_>, _: &Style<'_>, out: &mut dyn Write) -> fmt::Result {
    write!(out, "{}:", command.name())?;
    for arg in command.arguments() {
        write!(out, " {}", arg.id())?;
    }
    out.write_char('\n')
}

fn main() {
    let mut style = Style::plain();
    style.indent = 4;
    style.gap = 3;
    style.width = 64;
    let mut output = String::new();
    style.help(&CLI, &mut output).unwrap();
    print!("{output}");

    output.clear();
    style.help_layout = Some(compact);
    style.help(&CLI, &mut output).unwrap();
    print!("{output}");
}
