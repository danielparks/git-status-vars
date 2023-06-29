use std::cell::RefCell;
use std::fmt::{self, Debug, Display};
use std::io;
use std::rc::Rc;

/// A writer of var=value pairs.
///
/// See [`ShellWriter::new()`].
#[derive(Clone)]
pub struct ShellWriter<W: io::Write> {
    /// The output stream to write to.
    writer: Rc<RefCell<W>>,

    /// The prefix to add before every key, e.g. `"group_"` or `""`.
    prefix: String,
}

impl<W: io::Write> ShellWriter<W> {
    /// Create a new `ShellWriter`. The `prefix` will be prepended anytime a
    /// var is outputted, e.g. `prefixvar=value`.
    ///
    /// Generally, you will want to use this like:
    ///
    /// ```rust
    /// use git_status_vars::ShellWriter;
    /// ShellWriter::default().group("group").write_var("var", "value");
    /// // or...
    /// let mut buffer: Vec<u8> = vec![];
    /// ShellWriter::new(&mut buffer, "").group("group").write_var("var", "value");
    /// assert_eq!(buffer, b"group_var=value\n");
    /// ```
    #[must_use]
    pub fn new(writer: W, prefix: impl Display) -> Self {
        Self {
            writer: Rc::new(RefCell::new(writer)),
            prefix: prefix.to_string(),
        }
    }

    /// Write var=value with a value that was already quoted.
    fn write_raw(&self, var: impl Display, raw: impl Display) {
        writeln!(self.writer.borrow_mut(), "{}{}={}", self.prefix, var, raw)
            .unwrap();
    }

    /// Write var=value. `value` will be turned into a string, then quoted for
    /// safe shell insertion. `var` will be assumed to be a valid name for a
    /// shell variable.
    pub fn write_var(&self, var: impl Display, value: impl Display) {
        self.write_raw(var, shell_quote(value));
    }

    /// Write var=value. `value` will be formatted into a string using
    /// [`Debug`], then quoted for safe shell insertion. `var` will be assumed
    /// to be a valid name for a shell variable.
    pub fn write_var_debug(&self, var: impl Display, value: impl Debug) {
        self.write_raw(var, shell_quote_debug(value));
    }

    /// Write an object with the [`ShellVars`] trait. Mostly used with
    /// [`Self::group()`] and [`Self::group_n()`].
    pub fn write_vars(&self, vars: &impl ShellVars) {
        vars.write_to_shell(self);
    }

    /// Generate a sub-writer with this group name. Example output:
    ///
    /// ```sh
    /// prefix_group_var=value
    /// ```
    #[must_use]
    pub fn group(&self, group: impl Display) -> Self {
        Self {
            writer: self.writer.clone(),
            prefix: format!("{}{}_", self.prefix, group),
        }
    }

    /// Generate a sub-writer with this group name and number. Example output:
    ///
    /// ```sh
    /// prefix_groupN_var=value
    /// ```
    #[must_use]
    pub fn group_n(&self, prefix: impl Display, n: impl Display) -> Self {
        self.group(format!("{prefix}{n}"))
    }
}

impl ShellWriter<io::Stdout> {
    /// Create a new `ShellWriter` for [`io::stdout()`] and a prefix.
    #[must_use]
    pub fn with_prefix(prefix: impl Display) -> Self {
        Self::new(io::stdout(), prefix)
    }
}

impl Default for ShellWriter<io::Stdout> {
    /// Create a new `ShellWriter` for [`io::stdout()`] and no prefix.
    fn default() -> Self {
        Self::new(io::stdout(), "")
    }
}

impl<W: io::Write + Debug> Debug for ShellWriter<W> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("ShellWriter")
            .field("writer", &self.writer)
            .field("prefix", &self.prefix)
            .finish()
    }
}

/// An object that can be written as a group of shell variables.
pub trait ShellVars {
    /// Write `self` to the shell writer `out`.
    fn write_to_shell<W: io::Write>(&self, out: &ShellWriter<W>);
}

/// Quote a value for safe shell insertion.
///
/// ```rust
/// use git_status_vars::shell_quote;
/// assert_eq!(shell_quote("a $b `c`\nd"), "'a $b `c`\nd'");
/// ```
pub fn shell_quote(value: impl Display) -> String {
    shell_words::quote(&value.to_string()).into()
}

/// Format a value with [`Debug`] and quote it for safe shell insertion.
pub fn shell_quote_debug(value: impl Debug) -> String {
    shell_words::quote(&format!("{value:?}")).into()
}
