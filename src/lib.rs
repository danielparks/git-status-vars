use git_repository::head::Kind;
use std::fmt;
use std::path::Path;

#[derive(Debug, Default)]
pub struct Head {
    pub head_kind: String,
    pub ref_full: String,
    pub ref_kind: String,
    pub ref_name: String,
    pub ref2_full: String,
    pub ref2_kind: String,
    pub ref2_name: String,
    pub head_hash: String,
    pub head_hash_error: Option<git_repository::head::peel::Error>,
}

impl fmt::Display for Head {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "head_kind={}\n", self.head_kind)?;
        write!(f, "ref_full={}\n", self.ref_full)?;
        write!(f, "ref_kind={}\n", self.ref_kind)?;
        write!(f, "ref_name={}\n", self.ref_name)?;
        write!(f, "ref2_full={}\n", self.ref2_full)?;
        write!(f, "ref2_kind={}\n", self.ref2_kind)?;
        write!(f, "ref2_name={}\n", self.ref2_name)?;
        write!(f, "head_hash={}\n", self.head_hash)?;
        if let Some(error) = &self.head_hash_error {
            write!(f, "head_hash_error={:?}\n", error)?;
        } else {
            write!(f, "head_hash_error=\n")?;
        }

        Ok(())
    }
}

/// Print information about the HEAD of the repository at path.
pub fn head_info(path: impl AsRef<Path>) -> anyhow::Result<Head> {
    let mut output = Head::default();

    let repository = git_repository::discover(path)?;
    let head = repository.head()?;
    match head.kind {
        Kind::Symbolic(ref reference) => {
            output.head_kind = "symbolic".to_string();
            output.ref_full = reference.name.to_string();

            match reference.name.category_and_short_name() {
                Some((category, short_name)) => {
                    output.ref_kind = format!("{:?}", category);
                    output.ref_name = short_name.to_string();
                }
                None => {}
            }

            match &reference.target {
                git_ref::Target::Peeled(oid) => {
                    output.ref2_full = oid.to_string();
                }
                git_ref::Target::Symbolic(full_name) => {
                    output.ref2_full = full_name.to_string();

                    match full_name.category_and_short_name() {
                        Some((category, short_name)) => {
                            output.ref2_kind = format!("{:?}", category);
                            output.ref2_name = short_name.to_string();
                        }
                        None => {}
                    }
                }
            }
        }
        Kind::Unborn(ref full_name) => {
            output.head_kind = "unborn".to_string();
            output.ref_full = full_name.to_string();

            match full_name.category_and_short_name() {
                Some((category, short_name)) => {
                    output.ref_kind = format!("{:?}", category);
                    output.ref_name = short_name.to_string();
                }
                None => {}
            }
        }
        Kind::Detached {
            target: ref oid, ..
        } => {
            output.head_kind = "detached".to_string();
            output.ref_full = oid.to_string();
        }
    }

    match head.into_fully_peeled_id() {
        Some(Ok(id)) => {
            output.head_hash = id.detach().to_string();
        }
        Some(Err(error)) => {
            output.head_hash_error = Some(error);
        }
        None => {}
    }

    Ok(output)
}
