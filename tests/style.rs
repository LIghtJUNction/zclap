use core::fmt::{self, Write};
use zclap::{Arg, Command, Error, Style};

const CLI: Command = Command::new("copy")
    .about("Copy files with explicit options.")
    .args(&[
        Arg::flag("verbose")
            .short('v')
            .repeat()
            .help("Show each copied file"),
        Arg::value("mode")
            .short('m')
            .alias("format")
            .one_of(&["text", "raw"])
            .help("Choose a representation"),
        Arg::positional("source").required().help("Source file"),
    ]);

struct Fixed<const N: usize> {
    bytes: [u8; N],
    len: usize,
}

impl<const N: usize> Fixed<N> {
    fn new() -> Self {
        Self {
            bytes: [0; N],
            len: 0,
        }
    }

    fn as_str(&self) -> &str {
        core::str::from_utf8(&self.bytes[..self.len]).unwrap()
    }
}

impl<const N: usize> Write for Fixed<N> {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let end = self
            .len
            .checked_add(text.len())
            .filter(|&end| end <= N)
            .ok_or(fmt::Error)?;
        self.bytes[self.len..end].copy_from_slice(text.as_bytes());
        self.len = end;
        Ok(())
    }
}

#[test]
fn plain_help_uses_only_the_supplied_fixed_buffer() {
    let mut text = Fixed::<2048>::new();
    Style::plain().help(&CLI, &mut text).unwrap();
    let text = text.as_str();
    assert!(text.starts_with(
        "copy\nCopy files with explicit options.\n\nUsage: copy [OPTIONS] <source>\n"
    ));
    assert!(text.contains("Arguments:\n"));
    assert!(text.contains("Options:\n"));
    assert!(text.contains("-v, --verbose"));
    assert!(text.contains("--mode (alias: --format) <mode>"));
    assert!(text.contains("May be repeated."));
    assert!(text.contains("Choices: text raw"));
    assert!(!text.contains('\x1b'));
}

#[test]
fn default_style_and_arbitrary_prefixes_are_applied() {
    let mut ansi = String::new();
    Style::default().help(&CLI, &mut ansi).unwrap();
    assert!(ansi.contains("\x1b[1mcopy\x1b[0m"));
    assert!(ansi.contains("\x1b[36m--verbose\x1b[0m"));
    assert!(ansi.contains("\x1b[33m<source>\x1b[0m"));
    assert!(ansi.contains("\x1b[2mMay be repeated.\x1b[0m"));
    let mut style = Style::plain();
    style.title = ("<h>", "</h>");
    style.option = ("<o>", "</o>");
    style.value = ("<v>", "</v>");
    style.hint = ("<i>", "</i>");
    let mut text = String::new();
    style.help(&CLI, &mut text).unwrap();
    assert!(text.contains("<h>copy</h>"));
    assert!(text.contains("<o>--verbose</o>"));
    assert!(text.contains("<v><source></v>"));
    assert!(text.contains("<i>May be repeated.</i>"));
}

#[test]
fn colors_do_not_change_alignment() {
    let mut plain = String::new();
    let mut colored = String::new();
    Style::plain().help(&CLI, &mut plain).unwrap();
    let mut style = Style::plain();
    style.title = ("{T}", "{/T}");
    style.option = ("{O}", "{/O}");
    style.value = ("{V}", "{/V}");
    style.hint = ("{H}", "{/H}");
    style.help(&CLI, &mut colored).unwrap();
    for tag in ["{T}", "{/T}", "{O}", "{/O}", "{V}", "{/V}", "{H}", "{/H}"] {
        colored = colored.replace(tag, "");
    }
    assert_eq!(plain, colored);
}

#[test]
fn descriptions_wrap_and_preserve_paragraphs_at_unicode_boundaries() {
    const WRAP: Command = Command::new("x")
        .about("one two three\nfour\n\nfive")
        .args(&[Arg::flag("a").help("abcdefghi ééééé 中文字符 are preserved")]);
    let mut style = Style::plain();
    style.width = 18;
    style.indent = 1;
    style.gap = 1;
    let mut text = String::new();
    style.help(&WRAP, &mut text).unwrap();
    assert!(text.contains("four\n\nfive"));
    for line in text.lines() {
        assert!(line.chars().count() <= 18, "too wide: {line:?}");
    }
    assert!(text.contains("中文字符"));
    assert_eq!(text.chars().filter(|&c| c == 'é').count(), 5);
}

#[test]
fn unbounded_width_and_zero_spacing_are_supported() {
    const LONG: Command = Command::new("x").args(&[
        Arg::flag("long").help("A deliberately long description that remains on one line.")
    ]);
    let mut style = Style::plain();
    style.width = 0;
    style.indent = 0;
    style.gap = 0;
    let mut text = String::new();
    style.help(&LONG, &mut text).unwrap();
    assert!(text.contains("--longA deliberately long description that remains on one line.\n"));
}

#[test]
fn writer_errors_propagate_from_help_and_error() {
    assert_eq!(
        Style::plain().help(&CLI, &mut Fixed::<2>::new()),
        Err(fmt::Error)
    );
    let mut parser = CLI.parse(&[] as &[&str]);
    let _ = parser.next();
    let error = parser.finish().unwrap_err();
    assert_eq!(
        Style::plain().error(&error, &mut Fixed::<2>::new()),
        Err(fmt::Error)
    );
}

fn custom_help(command: &Command<'_>, style: &Style<'_>, out: &mut dyn Write) -> fmt::Result {
    write!(
        out,
        "{}{}{}: {} args",
        style.title.0,
        command.name(),
        style.title.1,
        command.arguments().len()
    )
}

fn custom_error(error: &Error<'_>, _: &Style<'_>, out: &mut dyn Write) -> fmt::Result {
    write!(out, "exit {}: {}", error.exit_code(), error)
}

#[test]
fn entire_help_and_error_layouts_are_replaceable() {
    let mut style = Style::plain();
    style.help_layout = Some(custom_help);
    style.error_layout = Some(custom_error);
    let mut text = String::new();
    style.help(&CLI, &mut text).unwrap();
    assert_eq!(text, "copy: 3 args");
    let mut parser = CLI.parse(&[] as &[&str]);
    let _ = parser.next();
    let error = parser.finish().unwrap_err();
    text.clear();
    style.error(&error, &mut text).unwrap();
    assert!(text.starts_with("exit 2: "));
    assert!(text.contains("source"));
}

#[test]
fn plain_and_colored_errors_include_a_newline() {
    let mut parser = CLI.parse(&[] as &[&str]);
    let _ = parser.next();
    let error = parser.finish().unwrap_err();
    let mut text = String::new();
    Style::plain().error(&error, &mut text).unwrap();
    assert_eq!(text, format!("error: {error}\n"));
    let mut colored = String::new();
    Style::default().error(&error, &mut colored).unwrap();
    assert_eq!(colored, format!("\x1b[31merror: {error}\x1b[0m\n"));
}

#[test]
fn subcommands_and_value_cardinality_are_visible() {
    const TREE: Command = Command::new("tree")
        .args(&[
            Arg::value("pair").values(2, 2).required(),
            Arg::value("range").values(1, 3),
            Arg::value("list").values(2, usize::MAX),
            Arg::positional("files").repeat(),
        ])
        .subcommands(&[
            Command::new("run").about("Run a job"),
            Command::new("status").about("Inspect the current job"),
        ]);
    const PLAIN: Style = Style::plain();
    let mut text = String::new();
    PLAIN.help(&TREE, &mut text).unwrap();
    assert!(text.contains("Usage: tree [OPTIONS] --pair <pair>{2} [files]... [COMMAND]"));
    assert!(text.contains("--pair <pair>{2} (required)"));
    assert!(text.contains("--range <range>{1..3}"));
    assert!(text.contains("--list <list>{2..}"));
    assert!(text.contains("Commands:\n  run"));
    assert!(text.contains("  status  Inspect the current job\n"));
}

#[test]
fn empty_help_and_trait_object_sinks_work() {
    let command = Command::new("empty");
    let mut text = Fixed::<64>::new();
    let sink: &mut dyn Write = &mut text;
    Style::plain().help(&command, sink).unwrap();
    assert_eq!(text.as_str(), "empty\n\nUsage: empty\n");
}

#[test]
fn all_aliases_are_rendered_in_declaration_order_without_changing_alignment() {
    const ALIASES: Command = Command::new("copy").args(&[
        Arg::value("output")
            .aliases(&["destination", "out", "書込"])
            .help("Destination file"),
        Arg::flag("verbose").help("Show progress"),
    ]);
    let mut plain = Fixed::<1024>::new();
    Style::plain().help(&ALIASES, &mut plain).unwrap();
    assert!(
        plain
            .as_str()
            .contains("--output (aliases: --destination, --out, --書込) <output>")
    );

    let mut colored = String::new();
    let mut style = Style::plain();
    style.option = ("<option>", "</option>");
    style.help(&ALIASES, &mut colored).unwrap();
    assert!(colored.contains("<option> (aliases: --destination, --out, --書込)</option>"));
    assert_eq!(
        plain.as_str(),
        colored.replace("<option>", "").replace("</option>", "")
    );
}

#[test]
fn a_single_alias_in_a_slice_preserves_the_existing_help_output() {
    const SINGLE: Command = Command::new("demo").args(&[Arg::value("mode")
        .short('m')
        .alias("format")
        .help("Choose a representation")]);
    const SLICE: Command = Command::new("demo").args(&[Arg::value("mode")
        .short('m')
        .aliases(&["format"])
        .help("Choose a representation")]);
    let mut single = String::new();
    let mut slice = String::new();
    Style::plain().help(&SINGLE, &mut single).unwrap();
    Style::plain().help(&SLICE, &mut slice).unwrap();
    assert!(single.contains("-m, --mode (alias: --format) <mode>"));
    assert_eq!(single, slice);
}

#[test]
fn replacing_and_clearing_aliases_updates_help() {
    const REPLACED: Command = Command::new("demo").args(&[
        Arg::flag("first")
            .aliases(&["stale", "old"])
            .alias("current"),
        Arg::flag("second").alias("removed").aliases(&[]),
    ]);
    let mut text = String::new();
    Style::plain().help(&REPLACED, &mut text).unwrap();
    assert!(text.contains("--first (alias: --current)"));
    assert!(text.contains("--second"));
    for removed in ["stale", "old", "removed"] {
        assert!(!text.contains(removed));
    }
}
