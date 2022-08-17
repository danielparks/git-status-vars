use clap::Parser;
use git2::Repository;
use git2::{ErrorClass, ErrorCode};
use git_summary::ShellWriter;
use simplelog::{
    ColorChoice, CombinedLogger, Config, ConfigBuilder, LevelFilter,
    TermLogger, TerminalMode,
};
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[clap(version, about)]
struct Params {
    /// Verbosity (may be repeated up to three times)
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,

    /// The repositories to summarize
    #[clap(parse(from_os_str))]
    repositories: Vec<PathBuf>,
}

fn main() {
    let params = Params::parse();

    let filter = match params.verbose {
        3.. => LevelFilter::Trace,
        2 => LevelFilter::Debug,
        1 => LevelFilter::Info,
        0 => LevelFilter::Warn,
    };

    CombinedLogger::init(vec![
        // Default logger
        new_term_logger(filter, new_logger_config().build()),
    ])
    .unwrap();

    let out = ShellWriter::default();

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

fn summarize_repository<W: std::io::Write>(
    out: &ShellWriter<W>,
    opened: Result<Repository, git2::Error>,
) {
    let result = match opened {
        Ok(repository) => summarize_opened_repository(out, repository),
        Err(error)
            if error.code() == ErrorCode::NotFound
                && error.class() == ErrorClass::Repository =>
        {
            out.write_var("repo_state", "NotFound");
            Ok(())
        }
        Err(error) => Err(error),
    };

    if let Err(error) = result {
        out.write_var("repo_state", "Error");
        out.write_var_debug("repo_error", &error);
    }
}

fn summarize_opened_repository<W: std::io::Write>(
    out: &ShellWriter<W>,
    repository: Repository,
) -> Result<(), git2::Error> {
    out.write_var_debug("repo_state", &repository.state());
    out.write_var("repo_empty", &repository.is_empty()?);
    out.write_var("repo_bare", &repository.is_bare());
    out.group("head")
        .write_vars(&git_summary::head_info(&repository)?);
    out.write_vars(&git_summary::count_changes(&repository)?);

    Ok(())
}

fn new_term_logger(level: LevelFilter, config: Config) -> Box<TermLogger> {
    TermLogger::new(level, config, TerminalMode::Mixed, ColorChoice::Auto)
}

fn new_logger_config() -> ConfigBuilder {
    let mut builder = ConfigBuilder::new();
    builder.set_time_to_local(true);
    builder.set_target_level(LevelFilter::Error);
    builder
}
