use core::fmt::{self, Write};

use crate::{Arg, Command, Error};

/// Independent, allocation-free help and error formatting.
///
/// Each token style is an arbitrary `(prefix, suffix)` pair. Use ANSI escapes,
/// markup, or empty strings. `width` wraps descriptions by Unicode scalar count;
/// wide terminal glyphs and combining marks do not have a portable display width.
/// A width of zero disables wrapping. Parsing never reads this configuration.
///
/// ```
/// use zclap::{Arg, Command, Style};
/// const CLI: Command = Command::new("copy")
///     .args(&[Arg::positional("source").required().help("Source file")]);
/// let mut style = Style::plain();
/// style.indent = 4;
/// style.title = ("\x1b[1m", "\x1b[0m");
/// let mut text = String::new();
/// style.help(&CLI, &mut text).unwrap();
/// assert!(text.contains("source"));
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Style<'a> {
    /// Headings and the command name.
    pub title: (&'a str, &'a str),
    /// Short and long option names, and subcommand names.
    pub option: (&'a str, &'a str),
    /// Value placeholders and positional names.
    pub value: (&'a str, &'a str),
    /// The error label and message.
    pub error: (&'a str, &'a str),
    /// Choices and repeatability hints.
    pub hint: (&'a str, &'a str),
    /// Indentation in spaces for argument, option, and command rows.
    pub indent: usize,
    /// Minimum spaces between a row's label and description.
    pub gap: usize,
    /// Description wrapping width; zero means unbounded.
    pub width: usize,
    /// Replace the complete help layout. Leave `None` for the built-in layout.
    #[allow(clippy::type_complexity)]
    pub help_layout: Option<fn(&Command<'_>, &Style<'_>, &mut dyn Write) -> fmt::Result>,
    /// Replace the complete error layout. Leave `None` for the built-in layout.
    #[allow(clippy::type_complexity)]
    pub error_layout: Option<fn(&Error<'_>, &Style<'_>, &mut dyn Write) -> fmt::Result>,
}

impl Style<'_> {
    /// Plain text with no ANSI sequences, two-space indentation, and width 80.
    pub const fn plain() -> Self {
        Self::PLAIN
    }

    const PLAIN: Self = Self {
        title: ("", ""),
        option: ("", ""),
        value: ("", ""),
        error: ("", ""),
        hint: ("", ""),
        indent: 2,
        gap: 2,
        width: 80,
        help_layout: None,
        error_layout: None,
    };

    /// Write help into a caller-owned formatter. Errors from the writer propagate.
    pub fn help(&self, command: &Command<'_>, out: &mut dyn Write) -> fmt::Result {
        if let Some(layout) = self.help_layout {
            return layout(command, self, out);
        }
        styled(out, self.title, command.name())?;
        out.write_char('\n')?;
        if !command.description().is_empty() {
            wrapped(out, command.description(), 0, 0, self.width)?;
            out.write_char('\n')?;
        }
        out.write_char('\n')?;
        styled(out, self.title, "Usage:")?;
        write!(out, " {}", command.name())?;
        if command
            .arguments()
            .iter()
            .any(|arg| !arg.is_positional() && !arg.is_required())
        {
            styled(out, self.hint, " [OPTIONS]")?;
        }
        for arg in command
            .arguments()
            .iter()
            .filter(|arg| !arg.is_positional() && arg.is_required())
        {
            out.write_char(' ')?;
            if let Some(long) = arg.long_name() {
                write!(out, "{}--{}{}", self.option.0, long, self.option.1)?;
            } else if let Some(short) = arg.short_name() {
                write!(out, "{}-{}{}", self.option.0, short, self.option.1)?;
            }
            if arg.takes_value() {
                out.write_char(' ')?;
                placeholder(out, self, arg)?;
            }
        }
        for arg in command.arguments().iter().filter(|arg| arg.is_positional()) {
            out.write_char(' ')?;
            placeholder(out, self, arg)?;
        }
        if !command.commands().is_empty() {
            styled(out, self.hint, " [COMMAND]")?;
        }
        out.write_char('\n')?;

        for positional in [true, false] {
            let mut any = false;
            let mut longest = 0;
            for arg in command
                .arguments()
                .iter()
                .filter(|arg| arg.is_positional() == positional)
            {
                any = true;
                let mut count = Count(0);
                label(&mut count, &Self::plain(), arg)?;
                longest = longest.max(count.0);
            }
            if !any {
                continue;
            }
            out.write_char('\n')?;
            styled(
                out,
                self.title,
                if positional { "Arguments:" } else { "Options:" },
            )?;
            out.write_char('\n')?;
            let column = self.description_column(longest);
            for arg in command
                .arguments()
                .iter()
                .filter(|arg| arg.is_positional() == positional)
            {
                spaces(out, self.indent)?;
                label(out, self, arg)?;
                let mut count = Count(0);
                label(&mut count, &Self::plain(), arg)?;
                self.description(out, arg.description(), count.0, column)?;
                if arg.is_repeat() || !arg.choices().is_empty() {
                    spaces(out, column)?;
                    out.write_str(self.hint.0)?;
                    if arg.is_repeat() {
                        out.write_str("May be repeated.")?;
                    }
                    if !arg.choices().is_empty() {
                        if arg.is_repeat() {
                            out.write_char(' ')?;
                        }
                        out.write_str("Choices:")?;
                        for choice in arg.choices() {
                            out.write_char(' ')?;
                            out.write_str(choice)?;
                        }
                    }
                    out.write_str(self.hint.1)?;
                    out.write_char('\n')?;
                }
            }
        }
        if !command.commands().is_empty() {
            out.write_char('\n')?;
            styled(out, self.title, "Commands:")?;
            out.write_char('\n')?;
            let longest = command
                .commands()
                .iter()
                .map(|cmd| cmd.name().chars().count())
                .max()
                .unwrap_or(0);
            let column = self.description_column(longest);
            for child in command.commands() {
                spaces(out, self.indent)?;
                styled(out, self.option, child.name())?;
                self.description(
                    out,
                    child.description(),
                    child.name().chars().count(),
                    column,
                )?;
            }
        }
        Ok(())
    }

    /// Write an error and a trailing newline. This method never exits the process.
    pub fn error(&self, error: &Error<'_>, out: &mut dyn Write) -> fmt::Result {
        if let Some(layout) = self.error_layout {
            return layout(error, self, out);
        }
        writeln!(out, "{}error: {}{}", self.error.0, error, self.error.1)
    }

    fn description_column(&self, longest: usize) -> usize {
        let desired = self.indent.saturating_add(longest).saturating_add(self.gap);
        if self.width == 0 {
            desired
        } else {
            desired.min((self.width / 2).max(self.indent.saturating_add(self.gap)))
        }
    }

    fn description(
        &self,
        out: &mut (impl Write + ?Sized),
        text: &str,
        label_len: usize,
        column: usize,
    ) -> fmt::Result {
        if !text.is_empty() {
            let end = self.indent.saturating_add(label_len);
            if end.saturating_add(self.gap) > column {
                out.write_char('\n')?;
                spaces(out, column)?;
            } else {
                spaces(out, column - end)?;
            }
            wrapped(out, text, column, column, self.width)?;
        }
        out.write_char('\n')
    }
}

impl Default for Style<'_> {
    /// ANSI colors are explicit; applications decide when a terminal supports them.
    fn default() -> Self {
        Self {
            title: ("\x1b[1m", "\x1b[0m"),
            option: ("\x1b[36m", "\x1b[0m"),
            value: ("\x1b[33m", "\x1b[0m"),
            error: ("\x1b[31m", "\x1b[0m"),
            hint: ("\x1b[2m", "\x1b[0m"),
            ..Self::plain()
        }
    }
}

struct Count(usize);

impl Write for Count {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        self.0 = self.0.saturating_add(text.chars().count());
        Ok(())
    }
}

fn styled(out: &mut (impl Write + ?Sized), pair: (&str, &str), text: &str) -> fmt::Result {
    out.write_str(pair.0)?;
    out.write_str(text)?;
    out.write_str(pair.1)
}

fn spaces(out: &mut (impl Write + ?Sized), count: usize) -> fmt::Result {
    const SPACES: &str = "                                                                ";
    let mut remaining = count;
    while remaining >= SPACES.len() {
        out.write_str(SPACES)?;
        remaining -= SPACES.len();
    }
    out.write_str(&SPACES[..remaining])
}

fn placeholder(out: &mut (impl Write + ?Sized), style: &Style<'_>, arg: &Arg<'_>) -> fmt::Result {
    out.write_str(style.value.0)?;
    let (open, close) = if arg.is_required() || !arg.is_positional() {
        ('<', '>')
    } else {
        ('[', ']')
    };
    write!(out, "{}{}{}", open, arg.id(), close)?;
    let (min, max) = arg.value_range();
    if arg.is_positional() && arg.is_repeat() && min == 1 {
        out.write_str("...")?;
    } else if max > 1 {
        if max == usize::MAX {
            write!(out, "{{{}..}}", min)?;
        } else if min == max {
            write!(out, "{{{}}}", min)?;
        } else {
            write!(out, "{{{}..{}}}", min, max)?;
        }
    }
    out.write_str(style.value.1)
}

fn label(out: &mut (impl Write + ?Sized), style: &Style<'_>, arg: &Arg<'_>) -> fmt::Result {
    if arg.is_positional() {
        return placeholder(out, style, arg);
    }
    if let Some(short) = arg.short_name() {
        write!(out, "{}-{}{}", style.option.0, short, style.option.1)?;
        if arg.long_name().is_some() {
            out.write_str(", ")?;
        }
    }
    if let Some(long) = arg.long_name() {
        write!(out, "{}--{}{}", style.option.0, long, style.option.1)?;
    }
    let aliases = arg.alias_names();
    if !aliases.is_empty() {
        write!(
            out,
            "{} ({}: ",
            style.option.0,
            if aliases.len() == 1 {
                "alias"
            } else {
                "aliases"
            }
        )?;
        for (index, alias) in aliases.iter().enumerate() {
            if index != 0 {
                out.write_str(", ")?;
            }
            write!(out, "--{alias}")?;
        }
        write!(out, "){}", style.option.1)?;
    }
    if arg.takes_value() {
        out.write_char(' ')?;
        placeholder(out, style, arg)?;
    }
    if arg.is_required() {
        styled(out, style.hint, " (required)")?;
    }
    Ok(())
}

fn wrapped(
    out: &mut (impl Write + ?Sized),
    text: &str,
    indent: usize,
    mut column: usize,
    width: usize,
) -> fmt::Result {
    for (line, paragraph) in text.split('\n').enumerate() {
        if line != 0 {
            out.write_char('\n')?;
            spaces(out, indent)?;
            column = indent;
        }
        for word in paragraph.split_whitespace() {
            let length = word.chars().count();
            let gap = usize::from(column > indent);
            if gap != 0 && width != 0 && column.saturating_add(gap).saturating_add(length) > width {
                out.write_char('\n')?;
                spaces(out, indent)?;
                column = indent;
            } else if gap != 0 {
                out.write_char(' ')?;
                column = column.saturating_add(1);
            }
            for ch in word.chars() {
                if width > indent && column >= width {
                    out.write_char('\n')?;
                    spaces(out, indent)?;
                    column = indent;
                }
                out.write_char(ch)?;
                column = column.saturating_add(1);
            }
        }
    }
    Ok(())
}
