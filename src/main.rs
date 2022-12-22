use clap::Parser;
use git2::Repository;
use git_status_vars::{summarize_repository, ShellWriter};
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[clap(version, about)]
struct Params {
    /// The repositories to summarize
    repositories: Vec<PathBuf>,

    /// Prefix for each shell var line (e.g. 'local ')
    #[clap(long, short = 'p')]
    prefix: Option<String>,
}

fn main() {
    let params = Params::parse();
    let out = ShellWriter::with_prefix(
        params.prefix.unwrap_or_else(|| String::from("")),
    );

    if params.repositories.is_empty() {
        summarize_repository(&out, Repository::open_from_env());
    } else if params.repositories.len() == 1 {
        summarize_repository(&out, Repository::open(&params.repositories[0]));
    } else {
        out.write_var("repo_count", params.repositories.len());
        for (i, repo_path) in params.repositories.iter().enumerate() {
            println!();
            let repo_out = &out.group_n("repo", i + 1);
            repo_out.write_var("path", repo_path.display());
            summarize_repository(repo_out, Repository::open(repo_path));
        }
    }
}
