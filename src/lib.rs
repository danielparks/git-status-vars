//! This is primarily a command line utility. The documentation for the command
//! line interface is in [README.md][].
//!
//! The primary entrance to this code is [`summarize_repository()`]. It opens a
//! [`Repository`], then calls [`summarize_opened_repository()`] on it.
//!
//! [README.md]: https://github.com/danielparks/git-status-vars/blob/main/README.md
use git2::Branch;
use git2::ReferenceType;
use git2::Repository;
use git2::{ErrorClass, ErrorCode};
use git2::{Status, StatusOptions, StatusShow};
use std::fmt;
use std::io;

mod shell_writer;
pub use shell_writer::*;

/// A reference in a git repository.
#[derive(Debug, Default)]
pub struct Reference {
    pub name: String,
    pub kind: String,
    pub error: String,
}

impl Reference {
    pub fn new<N, K>(name: N, kind: K) -> Self
    where
        N: AsRef<str>,
        K: AsRef<str>,
    {
        Reference {
            name: name.as_ref().to_string(),
            kind: kind.as_ref().to_string(),
            error: "".to_string(),
        }
    }

    pub fn new_with_error<N, K, E>(name: N, kind: K, error: E) -> Self
    where
        N: AsRef<str>,
        K: AsRef<str>,
        E: fmt::Debug,
    {
        Reference {
            name: name.as_ref().to_string(),
            kind: kind.as_ref().to_string(),
            error: format!("{:?}", error),
        }
    }

    pub fn symbolic(name: &str) -> Self {
        Reference::new(name, "symbolic")
    }

    pub fn direct(name: &str) -> Self {
        Reference::new(name, "direct")
    }

    pub fn short(&self) -> &str {
        self.name
            .strip_prefix("refs/heads/")
            .or_else(|| self.name.strip_prefix("refs/tags/"))
            .unwrap_or(&self.name)
    }
}

impl ShellVars for Reference {
    // Output the reference information with a prefix (e.g. "ref_").
    fn write_to_shell<W: io::Write>(&self, out: &ShellWriter<W>) {
        out.write_var("name", &self.name);
        out.write_var("short", self.short());
        out.write_var("kind", &self.kind);
        out.write_var("error", &self.error);
    }
}

/// The trail of a `HEAD` reference.
#[derive(Debug, Default)]
pub struct Head {
    pub trail: Vec<Reference>,
    pub hash: String,
    pub ahead_of_upstream: Option<usize>,
    pub behind_upstream: Option<usize>,
    pub upstream_error: String,
}

impl ShellVars for Head {
    fn write_to_shell<W: io::Write>(&self, out: &ShellWriter<W>) {
        out.write_var("ref_length", self.trail.len() - 1);
        for (i, reference) in self.trail[1..].iter().enumerate() {
            out.group_n("ref", i + 1).write_vars(reference);
        }
        out.write_var("hash", &self.hash);
        out.write_var("ahead", display_option(self.ahead_of_upstream));
        out.write_var("behind", display_option(self.behind_upstream));
        out.write_var("upstream_error", &self.upstream_error);
    }
}

/// Summarize information about a repository.
///
/// This takes the `Result` from one of the `Repository::open()` functions.
///
/// ### Example
///
/// ```no_run
/// use git_status_vars::{summarize_repository, ShellWriter};
/// use git2::Repository;
///
/// summarize_repository(&ShellWriter::default(), Repository::open_from_env());
/// ```
pub fn summarize_repository<W: std::io::Write>(
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
        out.write_var_debug("repo_error", error);
    }
}

/// Summarize information about a successfully opened repository.
///
/// ### Example
///
/// ```no_run
/// use git_status_vars::{summarize_opened_repository, ShellWriter};
/// use git2::Repository;
///
/// summarize_opened_repository(
///     &ShellWriter::default(),
///     Repository::open_from_env().unwrap(),
/// ).unwrap();
/// ```
pub fn summarize_opened_repository<W: std::io::Write>(
    out: &ShellWriter<W>,
    repository: Repository,
) -> Result<(), git2::Error> {
    out.write_var_debug("repo_state", repository.state());
    out.write_var(
        "repo_workdir",
        display_option(repository.workdir().map(|path| path.display())),
    );
    out.write_var("repo_empty", repository.is_empty()?);
    out.write_var("repo_bare", repository.is_bare());
    out.group("head").write_vars(&head_info(&repository)?);
    out.write_vars(&count_changes(&repository)?);

    Ok(())
}

/// Trace the `HEAD` reference for a repository.
pub fn head_info(repository: &Repository) -> Result<Head, git2::Error> {
    let mut current = "HEAD".to_string();
    let mut head = Head::default();
    loop {
        match repository.find_reference(&current) {
            Ok(reference) => match reference.kind() {
                Some(ReferenceType::Direct) => {
                    head.trail.push(Reference::direct(&display_option(
                        reference.name(),
                    )));
                    head.hash = display_option(reference.target());
                    break;
                }
                Some(ReferenceType::Symbolic) => {
                    head.trail.push(Reference::symbolic(&display_option(
                        reference.name(),
                    )));
                    let target = reference
                        .symbolic_target()
                        .expect("Symbolic ref should have symbolic target");
                    current = target.to_string();
                }
                None => {
                    head.trail.push(Reference::new(
                        display_option(reference.name()),
                        "unknown",
                    ));
                    break;
                }
            },
            Err(error) => {
                head.trail
                    .push(Reference::new_with_error(current, "", error));
                break;
            }
        };
    }

    match get_upstream_difference(repository) {
        Ok(Some((ahead, behind))) => {
            head.ahead_of_upstream = Some(ahead);
            head.behind_upstream = Some(behind);
        }
        Ok(None) => {}
        Err(error) => {
            head.upstream_error = format!("{:?}", error);
        }
    }

    Ok(head)
}

/// Get the (ahead, behind) count of HEAD versus its upstream branch.
pub fn get_upstream_difference(
    repository: &Repository,
) -> Result<Option<(usize, usize)>, git2::Error> {
    let local_ref = repository.head()?.resolve()?;
    if let Some(local_oid) = local_ref.target() {
        let upstream_branch = Branch::wrap(local_ref).upstream()?;
        if let Some(upstream_oid) = upstream_branch.get().target() {
            repository
                .graph_ahead_behind(local_oid, upstream_oid)
                .map(Some)
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn display_option(s: Option<impl fmt::Display>) -> String {
    s.map(|s| s.to_string()).unwrap_or_else(|| "".to_string())
}

/// Track changes in the working tree and index (staged area).
#[derive(Debug, Default)]
pub struct ChangeCounters {
    pub untracked: usize,
    pub unstaged: usize,
    pub staged: usize,
    pub conflicted: usize,
}

impl From<[usize; 4]> for ChangeCounters {
    fn from(array: [usize; 4]) -> Self {
        ChangeCounters {
            untracked: array[0],
            unstaged: array[1],
            staged: array[2],
            conflicted: array[3],
        }
    }
}

impl ShellVars for ChangeCounters {
    // Output the tree change information with a prefix (e.g. "tree_").
    fn write_to_shell<W: io::Write>(&self, out: &ShellWriter<W>) {
        out.write_var("untracked_count", self.untracked);
        out.write_var("unstaged_count", self.unstaged);
        out.write_var("staged_count", self.staged);
        out.write_var("conflicted_count", self.conflicted);
    }
}

/// Count changes in the working tree and index (staged area) of a repository.
pub fn count_changes(
    repository: &Repository,
) -> Result<ChangeCounters, git2::Error> {
    if repository.is_bare() {
        // Can't run status on bare repo.
        return Ok(ChangeCounters::default());
    }

    let mut options = StatusOptions::new();
    // exclude_submodules optional?
    options
        .show(StatusShow::IndexAndWorkdir)
        .include_untracked(true)
        .exclude_submodules(true);
    let statuses = repository.statuses(Some(&mut options))?;

    let mut counters: [usize; 4] = [0; 4];
    let buckets = [
        // Untracked
        Status::WT_NEW,
        // Working tree changed
        Status::WT_MODIFIED
            | Status::WT_DELETED
            | Status::WT_TYPECHANGE
            | Status::WT_RENAMED,
        // Staged
        Status::INDEX_NEW
            | Status::INDEX_MODIFIED
            | Status::INDEX_DELETED
            | Status::INDEX_RENAMED
            | Status::INDEX_TYPECHANGE,
        // Conflicted
        Status::CONFLICTED,
    ];

    for status in statuses.iter() {
        for (i, bits) in buckets.iter().enumerate() {
            if status.status().intersects(*bits) {
                counters[i] += 1;
            }
        }
    }

    Ok(ChangeCounters::from(counters))
}
