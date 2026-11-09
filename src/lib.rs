//! Allocation-free command-line parsing with borrowed events.
//!
//! Argument slices exclude the executable name. Consume events, then call
//! [`Parser::finish`] to validate required arguments and relationships.
//!
//! ```
//! use zclap::{Arg, Command, Event};
//! const CLI: Command = Command::new("demo").args(&[
//!     Arg::flag("verbose").short('v').repeat(),
//!     Arg::value("port").short('p'),
//!     Arg::positional("input").required(),
//! ]);
//! let argv = ["-vv", "--port=8080", "input.txt"];
//! let mut parser = CLI.parse(&argv);
//! let mut verbosity = 0;
//! while let Some(event) = parser.next()? {
//!     if let Event::Flag { arg: "verbose" } = event { verbosity += 1; }
//! }
//! parser.finish()?;
//! assert_eq!(verbosity, 2);
//! # Ok::<(), zclap::Error<'static>>(())
//! ```
//!
//! With `default-features = false`, only `core` is used. Definitions, parsing,
//! validation, errors and rendering allocate nothing. The `std` feature adds
//! borrowed `OsStr` / `OsString` input; collecting `std::env::args_os()` is the
//! application's responsibility.
#![no_std]
#![deny(unsafe_code)]
#![deny(missing_docs)]

#[cfg(feature = "std")]
extern crate std;

mod input;
mod style;

pub use style::Style;

use core::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Kind {
    Flag,
    Value,
    Positional,
}

/// A borrowed, const-constructible argument definition.
///
/// Named arguments have a long name equal to their ID unless `.long()` changes
/// it. Each argument has at most one alias, conflict and dependency; a later
/// call to the same builder replaces the earlier setting.
#[derive(Clone, Copy, Debug)]
pub struct Arg<'s> {
    id: &'s str,
    kind: Kind,
    short: Option<char>,
    long: Option<&'s str>,
    alias: Option<&'s str>,
    help: &'s str,
    choices: &'s [&'s str],
    conflict: Option<&'s str>,
    require: Option<&'s str>,
    required: bool,
    repeat: bool,
    min: usize,
    max: usize,
}

impl<'s> Arg<'s> {
    const fn new(id: &'s str, kind: Kind) -> Self {
        Self {
            id,
            kind,
            short: None,
            long: if matches!(kind, Kind::Positional) {
                None
            } else {
                Some(id)
            },
            alias: None,
            help: "",
            choices: &[],
            conflict: None,
            require: None,
            required: false,
            repeat: false,
            min: 1,
            max: 1,
        }
    }

    /// A flag that consumes no value.
    pub const fn flag(id: &'s str) -> Self {
        Self::new(id, Kind::Flag)
    }
    /// A named option that consumes one value per occurrence.
    pub const fn value(id: &'s str) -> Self {
        Self::new(id, Kind::Value)
    }
    /// A positional value, assigned in definition order.
    pub const fn positional(id: &'s str) -> Self {
        Self::new(id, Kind::Positional)
    }
    /// Set a short option name, without a leading dash.
    pub const fn short(mut self, name: char) -> Self {
        self.short = Some(name);
        self
    }
    /// Set the long option name, without leading dashes.
    pub const fn long(mut self, name: &'s str) -> Self {
        self.long = Some(name);
        self
    }
    /// Require at least one occurrence in this command's scope.
    pub const fn required(mut self) -> Self {
        self.required = true;
        self
    }
    /// Allow repeated occurrences; a repeated positional consumes the remainder.
    pub const fn repeat(mut self) -> Self {
        self.repeat = true;
        if matches!(self.kind, Kind::Positional) {
            self.max = usize::MAX;
        }
        self
    }
    /// Set descriptive help text. This never enters the parsing hot path.
    pub const fn help(mut self, text: &'s str) -> Self {
        self.help = text;
        self
    }
    /// Set one additional long name for the same argument.
    pub const fn alias(mut self, name: &'s str) -> Self {
        self.alias = Some(name);
        self
    }
    /// Restrict values to exact, case-sensitive UTF-8 strings. An empty list is unrestricted.
    pub const fn one_of(mut self, values: &'s [&'s str]) -> Self {
        self.choices = values;
        self
    }
    /// Reject this argument when the other argument ID is present, in either order.
    pub const fn conflicts(mut self, id: &'s str) -> Self {
        self.conflict = Some(id);
        self
    }
    /// Require the other argument ID when this argument is present.
    pub const fn requires(mut self, id: &'s str) -> Self {
        self.require = Some(id);
        self
    }
    /// Consume between `min` and `max` values per occurrence, both inclusive.
    ///
    /// `min` must be at least one. `usize::MAX` gives an unbounded maximum.
    /// A new option or, after `min` values, a subcommand ends the group. Put
    /// values beginning with a dash directly after `=` (or a short option).
    /// A positional with a variable range must be the last positional.
    pub const fn values(mut self, min: usize, max: usize) -> Self {
        self.min = min;
        self.max = max;
        self
    }
    /// Stable ID returned in events and used by relationship rules.
    pub const fn id(&self) -> &'s str {
        self.id
    }
    /// Short option spelling, when configured.
    pub const fn short_name(&self) -> Option<char> {
        self.short
    }
    /// Primary long option spelling, when configured.
    pub const fn long_name(&self) -> Option<&'s str> {
        self.long
    }
    /// Alternative long option spelling, when configured.
    pub const fn alias_name(&self) -> Option<&'s str> {
        self.alias
    }
    /// Descriptive help text.
    pub const fn description(&self) -> &'s str {
        self.help
    }
    /// Whether this argument is positional.
    pub const fn is_positional(&self) -> bool {
        matches!(self.kind, Kind::Positional)
    }
    /// Whether this argument consumes values.
    pub const fn takes_value(&self) -> bool {
        !matches!(self.kind, Kind::Flag)
    }
    /// Whether this argument must occur.
    pub const fn is_required(&self) -> bool {
        self.required
    }
    /// Whether repeated occurrences are accepted.
    pub const fn is_repeat(&self) -> bool {
        self.repeat
    }
    /// Inclusive minimum and maximum number of values per occurrence.
    pub const fn value_range(&self) -> (usize, usize) {
        (self.min, self.max)
    }
    /// Allowed values; an empty slice means unrestricted.
    pub const fn choices(&self) -> &'s [&'s str] {
        self.choices
    }
    /// ID of the mutually exclusive argument, when configured.
    pub const fn conflict_target(&self) -> Option<&'s str> {
        self.conflict
    }
    /// ID required when this argument appears, when configured.
    pub const fn requires_target(&self) -> Option<&'s str> {
        self.require
    }
}

/// A borrowed command definition. All builders support constant evaluation.
///
/// Subcommands have independent scopes. Put parent arguments before the child
/// command. Parent validation occurs when entering the child. There is no fixed
/// nesting limit and no implicit help/version flag.
#[derive(Clone, Copy, Debug)]
pub struct Command<'s> {
    name: &'s str,
    about: &'s str,
    args: &'s [Arg<'s>],
    commands: &'s [Command<'s>],
    definition_error: Option<Error<'s>>,
    rules: bool,
    has_alias: bool,
    // Compact, const-prepared metadata. Larger commands retain a linear fallback.
    long_order: [u16; 256],
    long_count: usize,
    required: [u64; 4],
    related: [u64; 4],
}

impl<'s> Command<'s> {
    /// Create an empty command.
    pub const fn new(name: &'s str) -> Self {
        Self {
            name,
            about: "",
            args: &[],
            commands: &[],
            definition_error: None,
            rules: false,
            has_alias: false,
            long_order: [0; 256],
            long_count: 0,
            required: [0; 4],
            related: [0; 4],
        }
    }
    /// Set descriptive text.
    pub const fn about(mut self, text: &'s str) -> Self {
        self.about = text;
        self
    }
    /// Set argument definitions. Invalid definitions become structured parse errors.
    pub const fn args(mut self, args: &'s [Arg<'s>]) -> Self {
        self.args = args;
        self.rules = false;
        self.has_alias = false;
        self.long_count = 0;
        self.required = [0; 4];
        self.related = [0; 4];
        let mut i = 0;
        while i < args.len() {
            self.has_alias = self.has_alias || args[i].alias.is_some();
            self.rules = self.rules
                || args[i].required
                || args[i].conflict.is_some()
                || args[i].require.is_some();
            if args.len() <= 256 {
                if args[i].required {
                    self.required[i / 64] |= 1 << (i % 64);
                }
                if args[i].conflict.is_some() || args[i].require.is_some() {
                    self.related[i / 64] |= 1 << (i % 64);
                }
                if let Some(name) = args[i].long {
                    let mut at = self.long_count;
                    while at > 0 {
                        let previous = self.long_order[at - 1] as usize;
                        let previous_name = match args[previous].long {
                            Some(s) => s,
                            None => "",
                        };
                        if !less(name, previous_name) {
                            break;
                        }
                        self.long_order[at] = self.long_order[at - 1];
                        at -= 1;
                    }
                    self.long_order[at] = i as u16;
                    self.long_count += 1;
                }
            }
            i += 1;
        }
        self.definition_error = self.check_definition();
        self
    }
    /// Set subcommands, which can themselves contain subcommands.
    pub const fn subcommands(mut self, commands: &'s [Command<'s>]) -> Self {
        self.commands = commands;
        self.definition_error = self.check_definition();
        self
    }
    /// Command name, also used as its subcommand spelling.
    pub const fn name(&self) -> &'s str {
        self.name
    }
    /// Descriptive text.
    pub const fn description(&self) -> &'s str {
        self.about
    }
    /// Arguments in definition order, for custom renderers or inspection.
    pub const fn arguments(&self) -> &'s [Arg<'s>] {
        self.args
    }
    /// Nested commands, for custom renderers or inspection.
    pub const fn commands(&self) -> &'s [Command<'s>] {
        self.commands
    }
    /// Check the definition independently of any input.
    pub const fn check(&self) -> Result<(), Error<'s>> {
        match self.check_definition() {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    /// Parse borrowed tokens, excluding the executable name.
    ///
    /// Supports `&str`, `&[u8]`, and, with `std`, `String`, `OsString`, `&OsStr`.
    /// Value events borrow the same representation as the input. The default
    /// inline state supports 256 definitions per command; use [`Self::parse_with`]
    /// for larger commands. No input-dependent allocations are made.
    pub fn parse<'c, 'a, T: input::Input>(&'c self, argv: &'a [T]) -> Parser<'c, 'a, T> {
        self.parse_with::<4, T>(argv)
    }

    /// Parse with `WORDS * 64` bits of inline presence state per command.
    ///
    /// ```
    /// # use zclap::Command;
    /// const CLI: Command = Command::new("large");
    /// let argv: [&str; 0] = [];
    /// let mut parser = CLI.parse_with::<8, _>(&argv); // 512 definitions
    /// parser.finish()?;
    /// # Ok::<(), zclap::Error<'static>>(())
    /// ```
    pub fn parse_with<'c, 'a, const WORDS: usize, T: input::Input>(
        &'c self,
        argv: &'a [T],
    ) -> Parser<'c, 'a, T, WORDS> {
        Parser::new(self, argv)
    }

    /// Explicit byte-input convenience; non-UTF-8 values are preserved.
    pub fn parse_bytes<'c, 'a>(&'c self, argv: &'a [&'a [u8]]) -> Parser<'c, 'a, &'a [u8]> {
        self.parse(argv)
    }

    /// Borrow OS arguments without converting, copying or allocating strings.
    #[cfg(feature = "std")]
    pub fn parse_os<'c, 'a>(
        &'c self,
        argv: &'a [std::ffi::OsString],
    ) -> Parser<'c, 'a, std::ffi::OsString> {
        self.parse(argv)
    }

    const fn check_definition(&self) -> Option<Error<'s>> {
        if !valid_name(self.name) {
            return Some(definition(self.name, "invalid command name"));
        }
        let mut i = 0;
        let mut optional_positional = false;
        let mut variable_positional = false;
        while i < self.args.len() {
            let arg = &self.args[i];
            if !valid_name(arg.id) {
                return Some(definition(arg.id, "invalid argument ID"));
            }
            if arg.min == 0 || arg.max < arg.min {
                return Some(definition(arg.id, "invalid value range"));
            }
            if let Some(c) = arg.short {
                if c == '-' || c == '=' || (c as u32) <= 32 || c == '\u{7f}' {
                    return Some(definition(arg.id, "invalid short name"));
                }
            }
            if let Some(name) = arg.long {
                if !valid_name(name) {
                    return Some(definition(arg.id, "invalid long name"));
                }
            }
            if let Some(name) = arg.alias {
                if !valid_name(name) || same_option(arg.long, Some(name)) {
                    return Some(definition(arg.id, "invalid or redundant alias"));
                }
            }
            if matches!(arg.kind, Kind::Flag)
                && (!arg.choices.is_empty() || arg.min != 1 || arg.max != 1)
            {
                return Some(definition(arg.id, "a flag cannot take values"));
            }
            if arg.is_positional() {
                if arg.short.is_some() || arg.long.is_some() || arg.alias.is_some() {
                    return Some(definition(arg.id, "a positional cannot have option names"));
                }
                if variable_positional {
                    return Some(definition(arg.id, "a variable positional must be last"));
                }
                if optional_positional && arg.required {
                    return Some(definition(
                        arg.id,
                        "required positional follows an optional one",
                    ));
                }
                optional_positional = !arg.required;
                variable_positional = arg.min != arg.max;
            }
            let mut j = 0;
            while j < i {
                let other = &self.args[j];
                if eq(arg.id, other.id) {
                    return Some(definition(arg.id, "duplicate argument ID"));
                }
                if let (Some(a), Some(b)) = (arg.short, other.short) {
                    if a == b {
                        return Some(definition(arg.id, "duplicate short name"));
                    }
                }
                if same_option(arg.long, other.long)
                    || same_option(arg.long, other.alias)
                    || same_option(arg.alias, other.long)
                    || same_option(arg.alias, other.alias)
                {
                    return Some(definition(arg.id, "duplicate long name or alias"));
                }
                j += 1;
            }
            if let Some(id) = arg.conflict {
                if eq(arg.id, id) || self.id_index(id).is_none() {
                    return Some(definition(arg.id, "invalid conflict target"));
                }
            }
            if let Some(id) = arg.require {
                if eq(arg.id, id) || self.id_index(id).is_none() {
                    return Some(definition(arg.id, "invalid dependency target"));
                }
            }
            i += 1;
        }
        i = 0;
        while i < self.commands.len() {
            let command = &self.commands[i];
            if !valid_name(command.name) {
                return Some(definition(command.name, "invalid subcommand name"));
            }
            if let Some(error) = command.definition_error {
                return Some(error);
            }
            let mut j = 0;
            while j < i {
                if eq(command.name, self.commands[j].name) {
                    return Some(definition(command.name, "duplicate subcommand"));
                }
                j += 1;
            }
            i += 1;
        }
        None
    }

    const fn id_index(&self, id: &str) -> Option<usize> {
        let mut i = 0;
        while i < self.args.len() {
            if eq(id, self.args[i].id) {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    fn long_index(&self, name: &[u8]) -> Option<usize> {
        let found = if self.long_count >= 16 {
            let mut left = 0;
            let mut right = self.long_count;
            let mut found = None;
            while left < right {
                let mid = left + (right - left) / 2;
                let i = self.long_order[mid] as usize;
                let candidate = self.args[i].long.unwrap_or("").as_bytes();
                match name.cmp(candidate) {
                    core::cmp::Ordering::Less => right = mid,
                    core::cmp::Ordering::Greater => left = mid + 1,
                    core::cmp::Ordering::Equal => {
                        found = Some(i);
                        break;
                    }
                }
            }
            found
        } else {
            self.args
                .iter()
                .position(|arg| arg.long.is_some_and(|s| s.as_bytes() == name))
        };
        found.or_else(|| {
            if self.has_alias {
                self.args
                    .iter()
                    .position(|arg| arg.alias.is_some_and(|s| s.as_bytes() == name))
            } else {
                None
            }
        })
    }

    fn short_index(&self, name: char) -> Option<usize> {
        self.args.iter().position(|arg| arg.short == Some(name))
    }

    fn child(&self, name: &[u8]) -> Option<&Command<'s>> {
        self.commands.iter().find(|c| c.name.as_bytes() == name)
    }
}

const fn eq(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

const fn less(a: &str, b: &str) -> bool {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let mut i = 0;
    while i < a.len() && i < b.len() {
        if a[i] != b[i] {
            return a[i] < b[i];
        }
        i += 1;
    }
    a.len() < b.len()
}

const fn same_option(a: Option<&str>, b: Option<&str>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => eq(a, b),
        _ => false,
    }
}

const fn valid_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.is_empty() || bytes[0] == b'-' {
        return false;
    }
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] <= b' ' || bytes[i] == b'=' || bytes[i] == 127 {
            return false;
        }
        i += 1;
    }
    true
}

const fn definition<'s>(arg: &'s str, reason: &'static str) -> Error<'s> {
    Error::Definition { arg, reason }
}

/// One borrowed event. Value representation follows the input (`str`, bytes or `OsStr`).
#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Event<'s, 'a, V: ?Sized = str> {
    /// One occurrence of a flag, including each member of a short cluster.
    Flag {
        /// Definition ID.
        arg: &'s str,
    },
    /// One value. Multi-value arguments produce one event per value.
    Value {
        /// Definition ID.
        arg: &'s str,
        /// Borrowed input, with option spelling and separator removed.
        value: &'a V,
    },
    /// Entered a subcommand after successfully validating its parent.
    Command {
        /// Selected command name.
        name: &'s str,
    },
}

impl<V: ?Sized> Copy for Event<'_, '_, V> {}
impl<V: ?Sized> Clone for Event<'_, '_, V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'s, 'a, V: ?Sized> Event<'s, 'a, V> {
    /// Argument ID, or the selected command name for a command event.
    pub fn id(&self) -> &'s str {
        match *self {
            Self::Flag { arg } | Self::Value { arg, .. } => arg,
            Self::Command { name } => name,
        }
    }
    /// Borrowed value, when this is a value event.
    pub fn value(&self) -> Option<&'a V> {
        match *self {
            Self::Value { value, .. } => Some(value),
            _ => None,
        }
    }
}

/// A small error containing borrowed definition names and zero-based token indices.
///
/// Errors do not copy or format input. Formatting is deferred to `Display` or
/// [`Style::error`]. The caller decides whether and how to terminate the process.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Error<'s> {
    /// An invalid command definition.
    Definition {
        /// Argument or command ID.
        arg: &'s str,
        /// Explanation independent of input.
        reason: &'static str,
    },
    /// More definitions than the selected inline state can hold.
    Capacity {
        /// Number of definitions needed.
        needed: usize,
        /// Number supported by this parser.
        available: usize,
    },
    /// An unknown option or invalid short-option encoding.
    Unknown {
        /// Zero-based token index (excludes executable name).
        index: usize,
    },
    /// No positional argument can consume this token.
    UnexpectedArgument {
        /// Zero-based token index.
        index: usize,
    },
    /// A flag was assigned a value.
    UnexpectedValue {
        /// Argument ID.
        arg: &'s str,
        /// Zero-based token index.
        index: usize,
    },
    /// An option or started positional group has too few values.
    MissingValue {
        /// Argument ID.
        arg: &'s str,
        /// Token where the group started.
        index: usize,
    },
    /// A non-repeatable argument appeared again.
    Duplicate {
        /// Argument ID.
        arg: &'s str,
        /// Zero-based token index.
        index: usize,
    },
    /// A value is outside the explicitly configured choices.
    InvalidValue {
        /// Argument ID.
        arg: &'s str,
        /// Zero-based token index.
        index: usize,
    },
    /// A required argument did not appear.
    MissingRequired {
        /// Argument ID.
        arg: &'s str,
    },
    /// Both mutually exclusive arguments appeared.
    Conflict {
        /// Argument declaring the conflict.
        arg: &'s str,
        /// Conflicting argument ID.
        other: &'s str,
    },
    /// A dependency was not supplied.
    Requires {
        /// Argument declaring the dependency.
        arg: &'s str,
        /// Missing argument ID.
        other: &'s str,
    },
    /// `finish` was called while unconsumed events remained.
    Unfinished {
        /// Index of the next unconsumed token.
        index: usize,
    },
}

impl Error<'_> {
    /// Conventional command-line usage-error exit code. No process is exited.
    pub const fn exit_code(&self) -> i32 {
        2
    }
    /// Input token index when the error refers to a specific token.
    pub const fn index(&self) -> Option<usize> {
        match *self {
            Self::Unknown { index }
            | Self::UnexpectedArgument { index }
            | Self::UnexpectedValue { index, .. }
            | Self::MissingValue { index, .. }
            | Self::Duplicate { index, .. }
            | Self::InvalidValue { index, .. }
            | Self::Unfinished { index } => Some(index),
            _ => None,
        }
    }
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Definition { arg, reason } => write!(out, "invalid definition '{arg}': {reason}"),
            Self::Capacity { needed, available } => write!(
                out,
                "{needed} arguments exceed parser capacity {available}; use parse_with"
            ),
            Self::Unknown { index } => write!(out, "unknown option at argument {index}"),
            Self::UnexpectedArgument { index } => {
                write!(out, "unexpected positional at argument {index}")
            }
            Self::UnexpectedValue { arg, index } => write!(
                out,
                "flag '{arg}' does not take a value at argument {index}"
            ),
            Self::MissingValue { arg, index } => {
                write!(out, "missing value for '{arg}' at argument {index}")
            }
            Self::Duplicate { arg, index } => {
                write!(out, "argument '{arg}' repeated at argument {index}")
            }
            Self::InvalidValue { arg, index } => {
                write!(out, "invalid value for '{arg}' at argument {index}")
            }
            Self::MissingRequired { arg } => write!(out, "missing required argument '{arg}'"),
            Self::Conflict { arg, other } => {
                write!(out, "argument '{arg}' conflicts with '{other}'")
            }
            Self::Requires { arg, other } => write!(out, "argument '{arg}' requires '{other}'"),
            Self::Unfinished { index } => write!(
                out,
                "unconsumed input at argument {index}; consume events before finish"
            ),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error<'_> {}

#[derive(Clone, Copy, Debug)]
struct Pending {
    arg: usize,
    start: usize,
    count: usize,
}

/// A single-pass, fallible event stream with inline presence state.
///
/// Every error is terminal: subsequent `next` or `finish` returns that same
/// error. Exhaustion is stable unless `finish` discovers a validation error.
/// Always call `finish` after consuming
/// all events to check required arguments, conflicts and dependencies.
pub struct Parser<'s, 'a, T: input::Input = &'a str, const WORDS: usize = 4> {
    command: &'s Command<'s>,
    argv: &'a [T],
    cursor: usize,
    seen: [u64; WORDS],
    short: Option<(usize, usize)>,
    pending: Option<Pending>,
    positional: usize,
    positional_count: usize,
    positional_start: usize,
    end_options: bool,
    done: bool,
    validated: bool,
    error: Option<Error<'s>>,
}

impl<'s, 'a, T: input::Input, const WORDS: usize> Parser<'s, 'a, T, WORDS> {
    fn new(command: &'s Command<'s>, argv: &'a [T]) -> Self {
        let mut parser = Self {
            command,
            argv,
            cursor: 0,
            seen: [0; WORDS],
            short: None,
            pending: None,
            positional: 0,
            positional_count: 0,
            positional_start: 0,
            end_options: false,
            done: false,
            validated: false,
            error: command.definition_error,
        };
        if !valid_name(command.name) {
            parser.error = Some(definition(command.name, "invalid command name"));
        }
        if command.args.len() > WORDS.saturating_mul(64) {
            parser.error = Some(Error::Capacity {
                needed: command.args.len(),
                available: WORDS.saturating_mul(64),
            });
        }
        parser
    }

    /// The currently selected command, useful for contextual help.
    pub fn command(&self) -> &'s Command<'s> {
        self.command
    }

    /// Produce one event, borrowing the original input. This is fallible rather
    /// than implementing `Iterator`, so `while let Some(e) = parser.next()?` works.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Result<Option<Event<'s, 'a, T::Value>>, Error<'s>> {
        if let Some(error) = self.error {
            return Err(error);
        }
        if self.done {
            return Ok(None);
        }
        match self.advance() {
            Ok(event) => Ok(event),
            Err(error) => self.fail(error),
        }
    }

    /// Validate the fully consumed command. Calling this again is harmless.
    /// Unconsumed input is an error, never silently discarded.
    pub fn finish(&mut self) -> Result<(), Error<'s>> {
        if let Some(error) = self.error {
            return Err(error);
        }
        if self.validated {
            return Ok(());
        }
        if self.cursor < self.argv.len() || self.short.is_some() {
            let index = self.short.map_or(self.cursor, |(i, _)| i);
            return self.fail(Error::Unfinished { index });
        }
        if let Some(pending) = self.pending {
            if pending.count < self.command.args[pending.arg].min {
                return self.fail(Error::MissingValue {
                    arg: self.command.args[pending.arg].id,
                    index: pending.start,
                });
            }
        }
        if let Err(error) = self.validate() {
            return self.fail(error);
        }
        self.done = true;
        self.validated = true;
        Ok(())
    }

    #[cold]
    fn fail<R>(&mut self, error: Error<'s>) -> Result<R, Error<'s>> {
        self.error = Some(error);
        Err(error)
    }

    fn present(&self, i: usize) -> bool {
        self.seen[i / 64] & (1 << (i % 64)) != 0
    }

    fn mark(&mut self, i: usize, index: usize) -> Result<(), Error<'s>> {
        let arg = &self.command.args[i];
        if !arg.repeat && self.present(i) {
            return Err(Error::Duplicate { arg: arg.id, index });
        }
        self.seen[i / 64] |= 1 << (i % 64);
        Ok(())
    }

    fn value_event(
        &self,
        i: usize,
        index: usize,
        offset: usize,
    ) -> Result<Event<'s, 'a, T::Value>, Error<'s>> {
        let arg = &self.command.args[i];
        let raw = &self.argv[index];
        let bytes = &raw.bytes()[offset..];
        if !arg.choices.is_empty() && !arg.choices.iter().any(|s| s.as_bytes() == bytes) {
            return Err(Error::InvalidValue { arg: arg.id, index });
        }
        Ok(Event::Value {
            arg: arg.id,
            value: raw.suffix(offset),
        })
    }

    fn start_option(
        &mut self,
        i: usize,
        index: usize,
        inline: Option<usize>,
    ) -> Result<Option<Event<'s, 'a, T::Value>>, Error<'s>> {
        let arg = &self.command.args[i];
        if matches!(arg.kind, Kind::Flag) && inline.is_some() {
            return Err(Error::UnexpectedValue { arg: arg.id, index });
        }
        self.mark(i, index)?;
        if matches!(arg.kind, Kind::Flag) {
            return Ok(Some(Event::Flag { arg: arg.id }));
        }
        let count = usize::from(inline.is_some());
        self.pending = if count < arg.max {
            Some(Pending {
                arg: i,
                start: index,
                count,
            })
        } else {
            None
        };
        match inline {
            Some(offset) => self.value_event(i, index, offset).map(Some),
            None => Ok(None),
        }
    }

    fn advance(&mut self) -> Result<Option<Event<'s, 'a, T::Value>>, Error<'s>> {
        loop {
            if let Some((index, offset)) = self.short {
                let token = self.argv[index].bytes();
                let (name, len) = short_char(&token[offset..]).ok_or(Error::Unknown { index })?;
                let i = self
                    .command
                    .short_index(name)
                    .ok_or(Error::Unknown { index })?;
                let next = offset + len;
                if self.command.args[i].takes_value() {
                    self.short = None;
                    let inline = if next < token.len() {
                        Some(next + usize::from(token[next] == b'='))
                    } else {
                        None
                    };
                    if let Some(event) = self.start_option(i, index, inline)? {
                        return Ok(Some(event));
                    }
                } else {
                    if token.get(next) == Some(&b'=') {
                        return Err(Error::UnexpectedValue {
                            arg: self.command.args[i].id,
                            index,
                        });
                    }
                    self.short = if next < token.len() {
                        Some((index, next))
                    } else {
                        None
                    };
                    return self.start_option(i, index, None);
                }
                continue;
            }
            if let Some(mut pending) = self.pending {
                let arg = &self.command.args[pending.arg];
                let token = self.argv.get(self.cursor).map(input::Input::bytes);
                let boundary = match token {
                    None => true,
                    Some(token) => {
                        is_option(token)
                            || (pending.count >= arg.min && self.command.child(token).is_some())
                    }
                };
                if boundary {
                    if pending.count < arg.min {
                        return Err(Error::MissingValue {
                            arg: arg.id,
                            index: pending.start,
                        });
                    }
                    self.pending = None;
                    continue;
                }
                let index = self.cursor;
                self.cursor += 1;
                pending.count += 1;
                self.pending = if pending.count < arg.max {
                    Some(pending)
                } else {
                    None
                };
                return self.value_event(pending.arg, index, 0).map(Some);
            }
            if self.cursor == self.argv.len() {
                self.done = true;
                return Ok(None);
            }
            let index = self.cursor;
            let token = self.argv[index].bytes();
            self.cursor += 1;
            if !self.end_options {
                if token == b"--" {
                    self.end_options = true;
                    continue;
                }
                if token.starts_with(b"--") {
                    let end = token.iter().position(|&b| b == b'=').unwrap_or(token.len());
                    let name = &token[2..end];
                    let i = self
                        .command
                        .long_index(name)
                        .ok_or(Error::Unknown { index })?;
                    let inline = if end < token.len() {
                        Some(end + 1)
                    } else {
                        None
                    };
                    if let Some(event) = self.start_option(i, index, inline)? {
                        return Ok(Some(event));
                    }
                    continue;
                }
                if is_option(token) {
                    self.short = Some((index, 1));
                    continue;
                }
                let in_required_group = self.positional_count > 0
                    && self.positional_count < self.command.args[self.positional].min;
                if let Some(child) = self.command.child(token).filter(|_| !in_required_group) {
                    self.validate()?;
                    if child.args.len() > WORDS.saturating_mul(64) {
                        return Err(Error::Capacity {
                            needed: child.args.len(),
                            available: WORDS.saturating_mul(64),
                        });
                    }
                    if let Some(error) = child.definition_error {
                        return Err(error);
                    }
                    self.command = child;
                    self.seen.fill(0);
                    self.positional = 0;
                    self.positional_count = 0;
                    return Ok(Some(Event::Command { name: child.name }));
                }
            }
            while self.positional < self.command.args.len()
                && !self.command.args[self.positional].is_positional()
            {
                self.positional += 1;
            }
            let i = self.positional;
            if i == self.command.args.len() {
                return Err(Error::UnexpectedArgument { index });
            }
            if self.positional_count == 0 {
                self.positional_start = index;
            }
            self.positional_count += 1;
            self.seen[i / 64] |= 1 << (i % 64);
            if self.positional_count == self.command.args[i].max {
                self.positional += 1;
                self.positional_count = 0;
            }
            return self.value_event(i, index, 0).map(Some);
        }
    }

    fn validate(&self) -> Result<(), Error<'s>> {
        if self.positional_count > 0
            && self.positional_count < self.command.args[self.positional].min
        {
            return Err(Error::MissingValue {
                arg: self.command.args[self.positional].id,
                index: self.positional_start,
            });
        }
        if !self.command.rules {
            return Ok(());
        }
        if self.command.args.len() <= 256 {
            for word in 0..WORDS.min(4) {
                let missing = self.command.required[word] & !self.seen[word];
                let mut candidates = missing | (self.command.related[word] & self.seen[word]);
                while candidates != 0 {
                    let bit = candidates.trailing_zeros() as usize;
                    let arg = &self.command.args[word * 64 + bit];
                    if missing & (1 << bit) != 0 {
                        return Err(Error::MissingRequired { arg: arg.id });
                    }
                    self.validate_relation(arg)?;
                    candidates &= candidates - 1;
                }
            }
            return Ok(());
        }
        for (i, arg) in self.command.args.iter().enumerate() {
            let present = self.present(i);
            if arg.required && !present {
                return Err(Error::MissingRequired { arg: arg.id });
            }
            if present {
                self.validate_relation(arg)?;
            }
        }
        Ok(())
    }

    fn validate_relation(&self, arg: &Arg<'s>) -> Result<(), Error<'s>> {
        if let Some(other) = arg.conflict {
            if self
                .command
                .id_index(other)
                .is_some_and(|i| self.present(i))
            {
                return Err(Error::Conflict { arg: arg.id, other });
            }
        }
        if let Some(other) = arg.require {
            if !self
                .command
                .id_index(other)
                .is_some_and(|i| self.present(i))
            {
                return Err(Error::Requires { arg: arg.id, other });
            }
        }
        Ok(())
    }
}

fn is_option(token: &[u8]) -> bool {
    token.len() > 1 && token[0] == b'-'
}

fn short_char(bytes: &[u8]) -> Option<(char, usize)> {
    let first = *bytes.first()?;
    if first.is_ascii() {
        return Some((first as char, 1));
    }
    let len = match first {
        0xc2..=0xdf => 2,
        0xe0..=0xef => 3,
        0xf0..=0xf4 => 4,
        _ => return None,
    };
    core::str::from_utf8(bytes.get(..len)?)
        .ok()?
        .chars()
        .next()
        .map(|c| (c, len))
}
