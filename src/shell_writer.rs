use std::cell::RefCell;
use std::fmt;
use std::io;
use std::rc::Rc;

#[derive(Clone)]
pub struct ShellWriter<W: io::Write> {
    writer: Rc<RefCell<W>>,
    prefix: String,
}

impl<W: io::Write> ShellWriter<W> {
    pub fn new(writer: W, prefix: impl fmt::Display) -> Self {
        Self {
            writer: Rc::new(RefCell::new(writer)),
            prefix: prefix.to_string(),
        }
    }

    pub fn write_var(&self, var: impl fmt::Display, value: impl fmt::Display) {
        write!(
            self.writer.borrow_mut(),
            "{}{}={}\n",
            self.prefix,
            var,
            shell_quote(value)
        )
        .unwrap();
    }

    pub fn write_var_debug(
        &self,
        var: impl fmt::Display,
        value: impl fmt::Debug,
    ) {
        write!(
            self.writer.borrow_mut(),
            "{}{}={}\n",
            self.prefix,
            var,
            shell_quote_debug(value)
        )
        .unwrap();
    }

    pub fn write_vars(&self, vars: &impl ShellVars) {
        vars.write_to_shell(self);
    }

    pub fn group(&self, prefix: impl fmt::Display) -> ShellWriter<W> {
        ShellWriter {
            writer: self.writer.clone(),
            prefix: format!("{}{}_", self.prefix, prefix),
        }
    }

    pub fn group_n(
        &self,
        prefix: impl fmt::Display,
        n: impl fmt::Display,
    ) -> ShellWriter<W> {
        self.group(format!("{}{}", prefix, n))
    }
}

impl Default for ShellWriter<io::Stdout> {
    fn default() -> Self {
        Self::new(io::stdout(), "")
    }
}

impl<W: io::Write> fmt::Debug for ShellWriter<W>
where
    W: fmt::Debug,
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

pub fn shell_quote(value: impl fmt::Display) -> String {
    shell_words::quote(&value.to_string()).into()
}

pub fn shell_quote_debug(value: impl fmt::Debug) -> String {
    shell_words::quote(&format!("{:?}", value)).into()
}
