use clap::Parser;
use git2::Repository;
use git2::{ErrorClass, ErrorCode};
use git_summary::{print_key_value, WriteEnv};
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

    if params.repositories.is_empty() {
        summarize_repository(Repository::open_from_env(), "");
    } else if params.repositories.len() == 1 {
        summarize_repository(Repository::open(&params.repositories[0]), "");
    } else {
        print_key_value("", "repo_count", params.repositories.len());
        for (i, repo_path) in params.repositories.iter().enumerate() {
            println!();
            let prefix = format!("repo{}_", i + 1);
            print_key_value(&prefix, "path", repo_path.display());
            summarize_repository(Repository::open(repo_path), &prefix);
        }
    }
}

fn summarize_repository(opened: Result<Repository, git2::Error>, prefix: &str) {
    let result = match opened {
        Ok(repository) => summarize_opened_repository(repository, prefix),
        Err(error)
            if error.code() == ErrorCode::NotFound
                && error.class() == ErrorClass::Repository =>
        {
            print_key_value(prefix, "repo_state", "NotFound");
            Ok(())
        }
        Err(error) => Err(error),
    };

    if let Err(error) = result {
        print_key_value(prefix, "repo_state", "Error");
        print_key_value(prefix, "repo_error", format!("{:?}", &error));
    }
}

fn summarize_opened_repository(
    repository: Repository,
    prefix: &str,
) -> Result<(), git2::Error> {
    print_key_value(prefix, "repo_state", format!("{:?}", &repository.state()));
    print_key_value(prefix, "repo_empty", &repository.is_empty()?);
    print_key_value(prefix, "repo_bare", &repository.is_bare());
    git_summary::head_info(&repository)?.print_env(format!("{}head_", prefix));
    git_summary::count_changes(&repository)?.print_env(prefix);

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
