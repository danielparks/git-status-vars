use clap::Parser;
use git2::Repository;
use git_summary::{shell_quote, shell_quote_debug};
use simplelog::{
    ColorChoice, CombinedLogger, Config, ConfigBuilder, LevelFilter,
    TermLogger, TerminalMode,
};
use std::path::PathBuf;
use std::process::exit;

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
    if let Err(error) = cli(Params::parse()) {
        eprintln!("# Error: {:#}", error);
        exit(1);
    }
}

fn cli(params: Params) -> anyhow::Result<()> {
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
        summarize_repository(&Repository::open_from_env()?)?;
    } else if params.repositories.len() == 1 {
        summarize_repository(&Repository::open(&params.repositories[0])?)?;
    } else {
        let mut blank = "";
        for repo_path in params.repositories {
            println!("{}# {}", blank, repo_path.display());
            blank = "\n";
            summarize_repository(&Repository::open(repo_path)?)?;
        }
    }

    Ok(())
}

fn summarize_repository(repository: &Repository) -> anyhow::Result<()> {
    println!("repo_state={}", shell_quote_debug(&repository.state()));
    println!("repo_empty={}", shell_quote(&repository.is_empty()?));
    println!("repo_bare={}", shell_quote(&repository.is_bare()));
    print!("{}", git_summary::head_info(&repository)?);
    print!("{}", git_summary::count_changes(&repository)?);

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
