use std::fmt;
use std::io;

pub fn shell_quote(value: impl fmt::Display) -> String {
    shell_words::quote(&format!("{}", value)).into()
}

pub fn shell_quote_debug(value: impl fmt::Debug) -> String {
    shell_words::quote(&format!("{:?}", value)).into()
}

pub fn write_key_value(
    out: &mut dyn io::Write,
    prefix: impl fmt::Display,
    key: impl fmt::Display,
    value: impl fmt::Display,
) -> io::Result<()> {
    write!(out, "{}{}={}\n", prefix, key, shell_quote(value))
}

pub fn print_key_value(
    prefix: impl fmt::Display,
    key: impl fmt::Display,
    value: impl fmt::Display,
) {
    write_key_value(&mut io::stdout(), prefix, key, value).unwrap();
}

pub trait WriteEnv {
    // Output the reference information with a prefix (e.g. "ref_").
    fn write_env(
        &self,
        out: &mut dyn io::Write,
        prefix: impl fmt::Display,
    ) -> io::Result<()>;

    // Print the reference information with a prefix (e.g. "ref_") to stdout.
    //
    // Panics if writing to `io::stdout` fails.
    fn print_env(&self, prefix: impl fmt::Display) {
        self.write_env(&mut io::stdout(), prefix).unwrap();
    }
}
