//! Parameters for executable.
//!
//! Needed by OS-specific code, so this has to be in its own module.

use std::path::PathBuf;

#[cfg(not(windows))]
use std::time::Duration;

/// Parameters to configure executable.
#[derive(Debug, clap::Parser)]
#[clap(version, about)]
pub struct Params {
    /// The repositories to summarize
    pub repositories: Vec<PathBuf>,

    /// Prefix for each shell var line (e.g. 'local ')
    #[clap(long, short = 'p')]
    pub prefix: Option<String>,

    /// Print timing information to stderr
    #[clap(short, long)]
    pub verbose: bool,

    /// Timeout:
    ///
    ///  - A number of seconds like "1.5".
    ///  - A duration like "1s", "200ms", or "2s 50ms".
    ///  - "none", 0, or "" for no timeout.
    #[cfg(not(windows))]
    #[clap(
        short,
        long,
        default_value = "500ms",
        value_parser = parse_duration,
        allow_hyphen_values = true, // Better error message.
    )]
    pub timeout: Duration,
}

/// Parse a duration from a parameter.
#[cfg(not(windows))]
fn parse_duration(input: &str) -> Result<Duration, clap::Error> {
    use clap::error::ErrorKind;
    use clap::CommandFactory;

    let input = input.trim();

    if input.is_empty() || input == "0" || input.to_lowercase() == "none" {
        // Kludge. Clap doesn’t let a value parser return `Option<...>`:
        // https://github.com/clap-rs/clap/discussions/5320
        return Ok(Duration::ZERO);
    }

    if input.starts_with('-') {
        Err(Params::command()
            .error(ErrorKind::InvalidValue, "duration cannot be negative"))
    } else if input.chars().all(|c| c.is_ascii_digit()) {
        // Input is all numbers, so assume it’s seconds.
        input
            .parse::<u64>()
            .map(Duration::from_secs)
            .map_err(|error| {
                Params::command().error(ErrorKind::InvalidValue, error)
            })
    } else {
        duration_str::parse(input).map_err(|error| {
            Params::command().error(ErrorKind::InvalidValue, error)
        })
    }
}
