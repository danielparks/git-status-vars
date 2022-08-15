use git2::ReferenceType;
use git2::Repository;
use std::fmt;

#[derive(Debug, Default)]
pub struct Reference {
    pub full_name: String,
    pub short_name: String,
    pub kind: String,
    pub error: String,
}

impl Reference {
    pub fn new<N, K, E>(full_name: N, kind: K, error: E) -> Self
    where
        N: AsRef<str>,
        K: AsRef<str>,
        E: fmt::Debug,
    {
        Reference {
            full_name: full_name.as_ref().to_string(),
            short_name: display_option(shorten(full_name.as_ref())),
            kind: kind.as_ref().to_string(),
            error: format!("{:?}", error),
        }
    }

    pub fn symbolic(full_name: &str) -> Self {
        Reference::new(full_name, "symbolic", "")
    }

    pub fn direct(full_name: &str) -> Self {
        Reference::new(full_name, "direct", "")
    }

    // Output the reference information with a prefix (e.g. "ref_").
    pub fn write_env(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: impl fmt::Display,
    ) -> fmt::Result {
        write_key_value(f, &prefix, "name", &self.full_name)?;
        write_key_value(f, &prefix, "short", &self.short_name)?;
        write_key_value(f, &prefix, "kind", &self.kind)?;
        write_key_value(f, &prefix, "error", &self.error)?;

        Ok(())
    }
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_env(f, "")
    }
}

#[derive(Debug, Default)]
pub struct Head {
    pub trail: Vec<Reference>,
    pub hash: String,
}

impl Head {
    // Output the head information with a prefix (e.g. "head_").
    pub fn write_env(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: impl fmt::Display,
    ) -> fmt::Result {
        write_key_value(f, &prefix, "ref_length", self.trail.len()-1)?;
        for (i, reference) in self.trail[1..].iter().enumerate() {
            reference.write_env(f, format!("{}ref{}_", &prefix, i+1))?;
        }
        write_key_value(f, &prefix, "hash", &self.hash)?;

        Ok(())
    }
}

impl fmt::Display for Head {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_env(f, "head_")
    }
}

fn write_key_value(
    f: &mut fmt::Formatter<'_>,
    prefix: impl fmt::Display,
    key: impl fmt::Display,
    value: impl fmt::Display,
) -> fmt::Result {
    write!(f, "{}{}={}\n", prefix, key, value)
}

/// Print information about the HEAD of the repository at path.
pub fn head_info(repository: &Repository) -> anyhow::Result<Head> {
    dbg!(repository.is_empty()?);
    dbg!(repository.is_bare());
    dbg!(repository.head_detached()?);
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
                        "",
                    ));
                    break;
                }
            },
            Err(error) => {
                head.trail.push(Reference::new(current, "", error));
                break;
            }
        };
    }

    Ok(head)
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
