use git_repository::head::Kind;
use std::fmt;
use std::path::Path;

#[derive(Debug, Default)]
pub struct Reference {
    pub full_name: String,
    pub short_name: String,
    pub kind: String,
}

impl Reference {
    // Output the reference with a prefix (e.g. "ref_").
    pub fn prefix_fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefix: &str,
    ) -> fmt::Result {
        write!(f, "{}full={}\n", prefix, self.full_name)?;
        write!(f, "{}short={}\n", prefix, self.short_name)?;
        write!(f, "{}kind={}\n", prefix, self.kind)?;

        Ok(())
    }
}

impl fmt::Display for Reference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.prefix_fmt(f, "")
    }
}

#[derive(Debug, Default)]
pub struct Head {
    pub ref1: Reference,
    pub ref2: Reference,
    pub head_kind: String,
    pub head_hash: String,
    pub head_hash_error: Option<git_repository::head::peel::Error>,
}

impl fmt::Display for Head {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "head_kind={}\n", self.head_kind)?;
        write!(f, "head_hash={}\n", self.head_hash)?;
        if let Some(error) = &self.head_hash_error {
            write!(f, "head_hash_error={:?}\n", error)?;
        } else {
            write!(f, "head_hash_error=\n")?;
        }

        self.ref1.prefix_fmt(f, "ref_")?;
        self.ref2.prefix_fmt(f, "ref2_")?;

        Ok(())
    }
}

impl From<&git_ref::Reference> for Reference {
    fn from(reference: &git_ref::Reference) -> Self {
        match reference.name.category_and_short_name() {
            Some((category, short_name)) => Reference {
                full_name: reference.name.to_string(),
                short_name: short_name.to_string(),
                kind: format!("{:?}", category),
            },
            None => Reference {
                full_name: reference.name.to_string(),
                ..Reference::default()
            },
        }
    }
}

impl From<&git_ref::FullName> for Reference {
    fn from(full_name: &git_ref::FullName) -> Self {
        match full_name.category_and_short_name() {
            Some((category, short_name)) => Reference {
                full_name: full_name.to_string(),
                short_name: short_name.to_string(),
                kind: format!("{:?}", category),
            },
            None => Reference {
                full_name: full_name.to_string(),
                ..Reference::default()
            },
        }
    }
}

impl From<&git_repository::ObjectId> for Reference {
    fn from(oid: &git_repository::ObjectId) -> Self {
        Reference {
            full_name: oid.to_string(),
            // FIXME short oid
            ..Reference::default()
        }
    }
}

/// Print information about the HEAD of the repository at path.
pub fn head_info(
    repository: &git_repository::Repository,
) -> anyhow::Result<Head> {
    let mut output = Head::default();

    let head = repository.head()?;
    match head.kind {
        Kind::Symbolic(ref reference) => {
            output.head_kind = "symbolic".to_string();
            output.ref1 = reference.into();

            match &reference.target {
                git_ref::Target::Peeled(oid) => {
                    output.ref2 = oid.into();
                }
                git_ref::Target::Symbolic(full_name) => {
                    output.ref2 = full_name.into();
                }
            }
        }
        Kind::Unborn(ref full_name) => {
            output.head_kind = "unborn".to_string();
            output.ref1 = full_name.into();
        }
        Kind::Detached {
            target: ref oid, ..
        } => {
            output.head_kind = "detached".to_string();
            output.ref1 = oid.into();
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
