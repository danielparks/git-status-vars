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
    let head = repository.head()?;
    // FIXME probably going to need to improve this
    // https://docs.rs/git2/latest/git2/struct.Reference.html
    // dbg!(head.target());
    // dbg!(head.target_peel());
    // dbg!(head.symbolic_target());
    // FIXME? can't distinguish between master and main when one is a symref
    // to the other.

    Ok(Head {
        full_name: head.name().unwrap_or("").to_string(),
        short_name: head.shorthand().unwrap_or("").to_string(),
        hash: head
            .target()
            .map(|t| t.to_string())
            .unwrap_or("".to_string()),
        kind: head
            .kind()
            .map(|k| format!("{:?}", k))
            .unwrap_or("".to_string()),
    })
}

pub fn tree_info(repository: &Repository) -> anyhow::Result<()> {
    Ok(())
}
