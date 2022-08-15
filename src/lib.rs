use git2::ReferenceType;
use git2::Repository;
use std::fmt;

#[derive(Debug, Default)]
pub struct Head {
    pub full_name: String,
    pub short_name: String,
    pub hash: String,
    pub kind: String,
}

impl Head {
    // Output the head information with a prefix (e.g. "head_").
    pub fn write_env(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
    ) -> fmt::Result {
        write!(f, "{}name={}\n", prefix, self.full_name)?;
        write!(f, "{}short={}\n", prefix, self.short_name)?;
        write!(f, "{}hash={}\n", prefix, self.hash)?;
        write!(f, "{}kind={}\n", prefix, self.kind)?;

        Ok(())
    }
}

impl fmt::Display for Head {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_env(f, "")
    }
}

/// Print information about the HEAD of the repository at path.
pub fn head_info(repository: &Repository) -> anyhow::Result<Head> {
    let head = repository.find_reference("HEAD")?;

    dbg!(repository.is_empty()?);
    dbg!(repository.is_bare());
    dbg!(repository.head_detached()?);
    dbg!(head.name());
    dbg!(head.target());
    dbg!(head.target_peel());
    dbg!(head.symbolic_target());
    dbg!(head.kind());
    // FIXME probably going to need to improve this
    // https://docs.rs/git2/latest/git2/struct.Reference.html
    // dbg!(head.target());
    // dbg!(head.target_peel());
    // dbg!(head.symbolic_target());
    // FIXME? can't distinguish between master and main when one is a symref
    // to the other.

    match head.kind() {
        Some(ReferenceType::Direct) => {
            // Detached HEAD
            Ok(Head {
                hash: display_option(head.target()),
                kind: "direct".to_string(),
                ..Head::default()
            })
        }
        Some(ReferenceType::Symbolic) => {
            let target = head
                .symbolic_target()
                .expect("Symbolic ref should have symbolic target");
            match repository.find_reference(target) {
                Ok(reference) => Ok(Head {
                    full_name: display_option(reference.name()),
                    short_name: display_option(reference.shorthand()),
                    hash: display_option(reference.target()),
                    kind: "symbolic".to_string(),
                    ..Head::default()
                }),
                Err(error) => {
                    // Invalid reference?
                    dbg!(error);
                    Ok(Head {
                        full_name: target.to_string(),
                        // FIXME calculate short_name?
                        kind: "symbolic".to_string(),
                        ..Head::default()
                    })
                }
            }
        }
        None => {
            // Uhhhh.
            Ok(Head {
                full_name: display_option(head.name()),
                short_name: display_option(head.shorthand()),
                hash: display_option(head.target()),
                kind: "unknown".to_string(),
                ..Head::default()
            })
        }
    }
}

/// Print information about the HEAD of the repository at path.
pub fn print_reference_trail(repository: &Repository, name: &str) {
    let mut current = name.to_string();
    loop {
        let reference = match repository.find_reference(&current) {
            Ok(reference) => reference,
            Err(error) => {
                println!("error: {:?}", error);
                return;
            }
        };

        match reference.kind() {
            Some(ReferenceType::Direct) => {
                println!("direct: {}", display_option(reference.target()));
                return;
            }
            Some(ReferenceType::Symbolic) => {
                let target = reference
                    .symbolic_target()
                    .expect("Symbolic ref should have symbolic target");
                match shorten(target) {
                    Some(short) => {
                        println!("symbolic: {} ({})", target, short);
                    }
                    None => {
                        println!("symbolic: {}", target);
                    }
                }
                current = target.to_string();
            }
            None => {
                println!("unknown: {}", display_option(reference.name()));
                return;
            }
        }
    }
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

fn debug_option<S>(s: Option<S>) -> String
where
    S: fmt::Debug,
{
    s.map(|s| format!("{:?}", s)).unwrap_or("".to_string())
}

pub fn tree_info(repository: &Repository) -> anyhow::Result<()> {
    Ok(())
}
