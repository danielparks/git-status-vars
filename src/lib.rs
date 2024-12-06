//! This is primarily a command line utility. The documentation for the command
//! line interface is in [README.md][].
//!
//! The primary entrance to this code is [`summarize_repository()`]. It opens a
//! [`Repository`], then calls [`summarize_opened_repository()`] on it.
//!
//! Currently the minimum supported Rust version (MSRV) is **1.74.1**.
//!
//! # Versioning
//!
//! This follows semantic versioning for the command line utility, not the crate
//! API. Breaking changes to the API are not guaranteed to involve a major
//! version change, since I don’t anticipate this being used as a crate by
//! anyone else.
//!
//! [README.md]: https://github.com/danielparks/git-status-vars/blob/main/README.md

// Most lint configuration is in lints.toml, but that isn’t supported by
// cargo-geiger, and it only supports deny, not forbid.
#![forbid(unsafe_code)]

use git2::Branch;
use git2::ReferenceType;
use git2::Repository;
use git2::{ErrorClass, ErrorCode};
use git2::{Status, StatusOptions, StatusShow};
use std::fmt;
use std::io;
use std::path::Path;

/// Manage outputting shell variables.
mod shell_writer;
pub use shell_writer::*;

/// A reference in a git repository.
#[derive(Debug, Default)]
pub struct Reference {
    /// The name of the reference, e.g. `"refs/heads/my_branch"`.
    pub name: String,

    /// The kind of reference, e.g. `"symbolic"` or `"direct"`.
    pub kind: String,

    /// An error encountered when trying to resolve the reference, or `""`.
    pub error: String,
}

impl Reference {
    /// Create a new reference without an error.
    #[must_use]
    pub fn new<N: fmt::Display, K: fmt::Display>(name: N, kind: K) -> Self {
        Self {
            name: name.to_string(),
            kind: kind.to_string(),
            error: "".to_owned(),
        }
    }

    /// Create a new reference with an error.
    #[must_use]
    pub fn new_with_error<N, K, E>(name: N, kind: K, error: E) -> Self
    where
        N: fmt::Display,
        K: fmt::Display,
        E: fmt::Debug,
    {
        Self {
            name: name.to_string(),
            kind: kind.to_string(),
            error: format!("{error:?}"),
        }
    }

    /// Create a new symbolic reference.
    #[must_use]
    pub fn symbolic<N: fmt::Display>(name: N) -> Self {
        Self::new(name, "symbolic")
    }

    /// Create a new direct reference.
    #[must_use]
    pub fn direct<N: fmt::Display>(name: N) -> Self {
        Self::new(name, "direct")
    }

    /// Get the short name of a reference if it’s a tag or branch. Otherwise,
    /// get the full name.
    #[must_use]
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
    /// The trail of references leading to the actual underlying commit.
    pub trail: Vec<Reference>,

    /// The hash of the commit.
    pub hash: String,

    /// How many commits are we ahead of upstream?
    ///
    /// `None` means that there is no upstream, or there is no equivalent branch
    /// in upstream.
    pub ahead_of_upstream: Option<usize>,

    /// How many commits are we behind upstream?
    ///
    /// `None` means that there is no upstream, or there is no equivalent branch
    /// in upstream.
    pub behind_upstream: Option<usize>,

    /// An error encountered trying to calculate differences with upstream.
    pub upstream_error: String,
}

impl ShellVars for Head {
    fn write_to_shell<W: io::Write>(&self, out: &ShellWriter<W>) {
        let trail = self.trail.get(1..).unwrap_or(&[]);
        out.write_var("ref_length", trail.len());
        for (i, reference) in trail.iter().enumerate() {
            // self.trail is actually 1 longer, so i + 1 always fits.
            #[allow(clippy::arithmetic_side_effects)]
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
/// # Example
///
/// ```no_run
/// use git_status_vars::{summarize_repository, ShellWriter};
/// use git2::Repository;
///
/// summarize_repository(&ShellWriter::default(), Repository::open_from_env());
/// ```
///
/// # Panics
///
/// This may panic if it can’t resolve a symbolic reference to a symbolic
/// target.
pub fn summarize_repository<W: std::io::Write>(
    out: &ShellWriter<W>,
    opened: Result<Repository, git2::Error>,
) {
    let result = match opened {
        Ok(repository) => summarize_opened_repository(out, &repository),
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
/// # Example
///
/// ```no_run
/// use git_status_vars::{summarize_opened_repository, ShellWriter};
/// use git2::Repository;
///
/// summarize_opened_repository(
///     &ShellWriter::default(),
///     &Repository::open_from_env().unwrap(),
/// ).unwrap();
/// ```
///
/// # Errors
///
/// This will return a [`git2::Error`] if there were problems getting repository
/// information. This is careful to load all repository information (and thus
/// encountering any errors) before generating any output.
///
/// # Panics
///
/// This may panic if it can’t resolve a symbolic reference to a symbolic
/// target.
pub fn summarize_opened_repository<W: std::io::Write>(
    out: &ShellWriter<W>,
    repository: &Repository,
) -> Result<(), git2::Error> {
    let state = repository.state();
    let workdir = display_option(repository.workdir().map(Path::display));
    let empty = repository.is_empty()?;
    let bare = repository.is_bare();
    let head = &head_info(repository);
    let changes = &count_changes(repository)?;

    out.write_var_debug("repo_state", state);
    out.write_var("repo_workdir", workdir);
    out.write_var("repo_empty", empty);
    out.write_var("repo_bare", bare);
    out.group("head").write_vars(head);
    out.write_vars(changes);

    Ok(())
}

/// Trace the `HEAD` reference for a repository.
///
/// # Panics
///
/// This may panic if it can’t resolve a symbolic reference to a symbolic
/// target.
#[allow(clippy::similar_names)]
#[must_use]
pub fn head_info(repository: &Repository) -> Head {
    let mut current = "HEAD".to_owned();
    let mut head = Head::default();
    loop {
        match repository.find_reference(&current) {
            Ok(reference) => match reference.kind() {
                Some(ReferenceType::Direct) => {
                    head.trail.push(Reference::direct(display_option(
                        reference.name(),
                    )));
                    head.hash = display_option(reference.target());
                    break;
                }
                Some(ReferenceType::Symbolic) => {
                    head.trail.push(Reference::symbolic(display_option(
                        reference.name(),
                    )));
                    let target = reference
                        .symbolic_target()
                        .expect("Symbolic ref should have symbolic target");
                    target.clone_into(&mut current);
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
            head.upstream_error = format!("{error:?}");
        }
    }

    head
}

/// Get the (ahead, behind) count of HEAD versus its upstream branch.
///
/// # Errors
///
/// This will return [`git2::Error`] if there were problems resolving the
/// the repository head, or if there was an error finding the upstream branch
/// (but it will return `Ok(None)` if there simply is no upstream or upstream
/// branch).
pub fn get_upstream_difference(
    repository: &Repository,
) -> Result<Option<(usize, usize)>, git2::Error> {
    let local_ref = repository.head()?.resolve()?;
    if let Some(local_oid) = local_ref.target() {
        Branch::wrap(local_ref)
            .upstream()?
            .get()
            .target()
            .map(|upstream_oid| {
                repository.graph_ahead_behind(local_oid, upstream_oid)
            })
            .transpose()
    } else {
        Ok(None)
    }
}

/// Format `Option<impl fmt::Display>` for display. `None` becomes `""`.
fn display_option<V: fmt::Display>(s: Option<V>) -> String {
    s.map(|s| s.to_string()).unwrap_or_else(|| "".to_owned())
}

/// Track changes in the working tree and index (staged area).
#[derive(Debug, Default)]
pub struct ChangeCounters {
    /// The number of untracked files (not in the index).
    pub untracked: usize,

    /// The number of files that have been modified, but haven’t been staged.
    pub unstaged: usize,

    /// The number of files that have been staged.
    pub staged: usize,

    /// The number of files with conflicts.
    pub conflicted: usize,
}

impl From<[usize; 4]> for ChangeCounters {
    fn from(array: [usize; 4]) -> Self {
        Self {
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
///
/// # Errors
///
/// This will return [`git2::Error`] if there was an error getting status
/// information from the repository.
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
                counters[i] = counters[i].saturating_add(1);
            }
        }
    }

    Ok(ChangeCounters::from(counters))
}
