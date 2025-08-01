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

/// Signal handler for SIGALRM (for timeout).
extern "C" fn sigalrm_handler(_: nix::libc::c_int) {
    let _ = write(std::io::stdout(), b"repo_state=GitStatusTimedOut\n");
    let _ = write(std::io::stderr(), b"git-status-vars timed out\n");
    // Safety: FIXME
    unsafe {
        libc::_exit(1);
    }
}

fn main() {
    let params = Params::parse();
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

    let alarm_action = SigAction::new(
        SigHandler::Handler(sigalrm_handler),
        SaFlags::empty(),
        SigSet::empty(),
    );
    // Safety: FIXME
    unsafe {
        let _ = sigaction(Signal::SIGALRM, &alarm_action);
    }

    if params.timeout != 0 {
        nix::unistd::alarm::set(params.timeout);
    }

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
