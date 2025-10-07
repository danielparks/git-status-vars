//! git-status-vars executable.

use clap::Parser;
use git2::Repository;
use git_status_vars::{summarize_repository, ShellWriter};
use std::io;

mod params;
use params::Params;

#[cfg(not(windows))]
mod unix;
#[cfg(not(windows))]
use unix::{os_exit_hook, os_params_hook};

#[cfg(windows)]
mod windows;
#[cfg(windows)]
use windows::{os_exit_hook, os_params_hook};

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

    let os_state = os_params_hook(&params);

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

    os_exit_hook(os_state);
}
