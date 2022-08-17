use std::cell::RefCell;
use std::fmt::{self, Debug, Display};
use std::io;
use std::rc::Rc;

#[derive(Clone)]
pub struct ShellWriter<W: io::Write> {
    writer: Rc<RefCell<W>>,
    prefix: String,
}

impl<W: io::Write> ShellWriter<W> {
    pub fn new(writer: W, prefix: impl Display) -> Self {
        Self {
            writer: Rc::new(RefCell::new(writer)),
            prefix: prefix.to_string(),
        }
    }

    fn write_raw(&self, var: impl Display, raw: impl Display) {
        writeln!(self.writer.borrow_mut(), "{}{}={}", self.prefix, var, raw)
            .unwrap();
    }

    pub fn write_var(&self, var: impl Display, value: impl Display) {
        self.write_raw(var, shell_quote(value));
    }

    pub fn write_var_debug(&self, var: impl Display, value: impl Debug) {
        self.write_raw(var, shell_quote_debug(value));
    }

    pub fn write_vars(&self, vars: &impl ShellVars) {
        vars.write_to_shell(self);
    }

    pub fn group(&self, prefix: impl Display) -> ShellWriter<W> {
        ShellWriter {
            writer: self.writer.clone(),
            prefix: format!("{}{}_", self.prefix, prefix),
        }
    }

    pub fn group_n(
        &self,
        prefix: impl Display,
        n: impl Display,
    ) -> ShellWriter<W> {
        self.group(format!("{}{}", prefix, n))
    }
}

impl Default for ShellWriter<io::Stdout> {
    fn default() -> Self {
        Self::new(io::stdout(), "")
    }
}

impl<W: io::Write> Debug for ShellWriter<W>
where
    W: Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("ShellWriter")
            .field("writer", &self.writer)
            .field("prefix", &self.prefix)
            .finish()
    }
}

pub trait ShellVars {
    fn write_to_shell<W: io::Write>(&self, out: &ShellWriter<W>);
}

pub fn shell_quote(value: impl Display) -> String {
    shell_words::quote(&value.to_string()).into()
}

pub fn shell_quote_debug(value: impl Debug) -> String {
    shell_words::quote(&format!("{:?}", value)).into()
}
