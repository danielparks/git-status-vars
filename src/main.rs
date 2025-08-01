//! git-status-vars executable.

use clap::Parser;
use git2::Repository;
use git_status_vars::{summarize_repository, ShellWriter};
use nix::sys::signal::{
    sigaction, SaFlags, SigAction, SigHandler, SigSet, Signal,
};
use nix::unistd::write;
use std::io;
use std::path::PathBuf;

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

    /// Timeout in seconds (0 means no timeout)
    #[clap(short, long, default_value = "1")]
    pub timeout: u32,
}

fn main() {
    let params = Params::parse();
    install_timeout(params.timeout);

    let out = ShellWriter::with_prefix(params.prefix.unwrap_or_default());

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
}

/// Set up the timeout.
///
/// This uses [`alarm()`] and a signal handler to kill this process if it takes
/// too long (more than `timeout` seconds).
///
/// [`alarm()`]: https://man7.org/linux/man-pages/man2/alarm.2.html
fn install_timeout(timeout: u32) {
    if timeout == 0 {
        return;
    }

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

    nix::unistd::alarm::set(timeout);
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
