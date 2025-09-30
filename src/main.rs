//! git-status-vars executable.

use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};
use git2::Repository;
use git_status_vars::{summarize_repository, ShellWriter};
use nix::sys::signal::{
    kill, sigaction, SaFlags, SigAction, SigHandler, SigSet, Signal,
};
use nix::unistd::{fork, getpid, write, ForkResult, Pid};
use std::path::PathBuf;
use std::time::Duration;
use std::{io, thread};

/// Parameters to configure executable.
#[derive(Debug, clap::Parser)]
#[clap(version, about)]
struct Params {
    /// The repositories to summarize
    repositories: Vec<PathBuf>,

    /// Prefix for each shell var line (e.g. 'local ')
    #[clap(long, short = 'p')]
    prefix: Option<String>,

    /// Print timing information to stderr
    #[clap(short, long)]
    pub verbose: bool,

    /// Timeout:
    ///
    ///  - A number of seconds like "1.5".
    ///  - A duration like "1s", "200ms", or "2s 50ms".
    ///  - "none", 0, or "" for no timeout.
    #[clap(
        short,
        long,
        default_value = "500ms",
        value_parser = parse_duration,
        allow_hyphen_values = true, // Better error message.
    )]
    pub timeout: Duration,
}

fn main() {
    let params = Params::parse();
    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_target(false)
        .with_max_level(if params.verbose {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .compact()
        .init();

    // Kludge. Clap doesn’t let a value parser return `Option<...>`:
    // https://github.com/clap-rs/clap/discussions/5320
    let child = (params.timeout == Duration::ZERO)
        .then(|| install_timeout(params.timeout));

    let out = ShellWriter::with_prefix(params.prefix.unwrap_or_default());

    if params.repositories.is_empty() {
        summarize_repository(&out, Repository::open_from_env());
    } else if params.repositories.len() == 1 {
        summarize_repository(&out, Repository::open(&params.repositories[0]));
    } else {
        out.write_var("repo_count", params.repositories.len());
        for (i, repo_path) in params.repositories.iter().enumerate() {
            println!();
            let repo_out = &out.group_n("repo", i.wrapping_add(1));
            repo_out.write_var("path", repo_path.display());
            summarize_repository(repo_out, Repository::open(repo_path));
        }
    }

    if let Some(child) = child {
        tracing::debug!("Killing child {child} used for timeout");
        // Ignore errors; the child will die on its own eventually.
        let _ = kill(child, Signal::SIGTERM);
    }
}

/// Parse a duration from a parameter.
fn parse_duration(input: &str) -> Result<Duration, clap::Error> {
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

/// Set up the timeout.
///
/// This uses [`alarm()`] and a signal handler to kill this process if it takes
/// too long (more than `timeout` seconds).
///
/// [`alarm()`]: https://man7.org/linux/man-pages/man2/alarm.2.html
fn install_timeout(timeout: Duration) -> Pid {
    let alarm_action = SigAction::new(
        SigHandler::Handler(sigalrm_handler),
        SaFlags::empty(),
        SigSet::empty(),
    );

    // Safety: see `sigaction` documentation:
    //
    //   1. The signal handler only uses async-signal-safe functions.
    //   2. We ignore the old signal handler.
    unsafe {
        let _ = sigaction(Signal::SIGALRM, &alarm_action);
    }

    let parent = getpid();

    // Safety: this is not a multi-threaded application, which is the only issue
    // mentioned in the `nix::unistd::fork()` docs.
    let child = match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => child,
        Ok(ForkResult::Child) => {
            // Not multithreaded, so non-async-safe syscalls are fine to use.
            thread::sleep(timeout);

            // Ignore errors. Most likely the parent process already exited.
            let _ = kill(parent, Signal::SIGALRM);

            // Don't run atexit handlers since this is the child process.
            // Safety: libc has no docs. Not multithreaded, so no issues there.
            unsafe {
                libc::_exit(0);
            }
        }
        Err(error) => panic!("Could not fork child for timeout: {error}"),
    };
    tracing::debug!("Forked child {child} to send SIGALRM after {timeout:?}");
    child
}

/// Signal handler for SIGALRM (for timeout).
///
/// This only calls [async-signal-safe] functions.
///
/// [async-signal-safe]: https://man7.org/linux/man-pages/man7/signal-safety.7.html
extern "C" fn sigalrm_handler(_: nix::libc::c_int) {
    // Start with a newline in case we were in the middle of printing a line.
    let _ = write(
        std::io::stdout(),
        b"\nrepo_state=Error\nrepo_error='Timed out'\n",
    );

    // Safety: some things might not be cleaned up. However:
    //
    //   1. We shouldn’t be making changes to the git repo.
    //   2. Even if we were making changes, git is resilient to being killed in
    //      the middle of an operation.
    unsafe {
        libc::_exit(2);
    }
}
