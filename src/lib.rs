use git2::ReferenceType;
use git2::Repository;
use git2::{Status, StatusOptions, StatusShow};
use std::fmt;
use std::io;

mod write_env;
pub use write_env::*;

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
        shorten(&self.name).unwrap_or(&self.name)
    }
}

impl WriteEnv for Reference {
    // Output the reference information with a prefix (e.g. "ref_").
    fn write_env(
        &self,
        out: &mut dyn io::Write,
        prefix: impl fmt::Display,
    ) -> io::Result<()> {
        write_key_value(out, &prefix, "name", &self.name)?;
        write_key_value(out, &prefix, "short", &self.short())?;
        write_key_value(out, &prefix, "kind", &self.kind)?;
        write_key_value(out, &prefix, "error", &self.error)?;

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Head {
    pub trail: Vec<Reference>,
    pub hash: String,
}

impl WriteEnv for Head {
    // Output the head information with a prefix (e.g. "head_").
    fn write_env(
        &self,
        out: &mut dyn io::Write,
        prefix: impl fmt::Display,
    ) -> io::Result<()> {
        write_key_value(out, &prefix, "ref_length", self.trail.len() - 1)?;
        for (i, reference) in self.trail[1..].iter().enumerate() {
            reference.write_env(out, format!("{}ref{}_", &prefix, i + 1))?;
        }
        write_key_value(out, &prefix, "hash", &self.hash)?;

        Ok(())
    }
}

/// Print information about the HEAD of the repository at path.
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
                        &display_option(reference.name()),
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

    Ok(head)
}

// Shorten a reference if possible.
//
// Does not normalize the reference first. Requires UTF-8. Does not check for
// conflicts (e.g. if there are branches or tags with the same name).
pub fn shorten(full_name: &str) -> Option<&str> {
    full_name
        .strip_prefix("refs/heads/")
        .or_else(|| full_name.strip_prefix("refs/tags/"))
}

fn display_option<S>(s: Option<S>) -> String
where
    S: fmt::Display,
{
    s.map(|s| s.to_string()).unwrap_or("".to_string())
}

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

impl WriteEnv for ChangeCounters {
    // Output the tree change information with a prefix (e.g. "ref_").
    fn write_env(
        &self,
        out: &mut dyn io::Write,
        prefix: impl fmt::Display,
    ) -> io::Result<()> {
        write_key_value(out, &prefix, "untracked_count", &self.untracked)?;
        write_key_value(out, &prefix, "unstaged_count", &self.unstaged)?;
        write_key_value(out, &prefix, "staged_count", &self.staged)?;
        write_key_value(out, &prefix, "conflicted_count", &self.conflicted)?;

        Ok(())
    }
}

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
