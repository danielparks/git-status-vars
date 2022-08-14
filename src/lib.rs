use git_repository::head::Kind;
use std::path::Path;

/// Print information about the HEAD of the repository at path.
pub fn head_info(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let repository = git_repository::discover(path)?;
    let head = repository.head()?;
    match head.kind {
        Kind::Symbolic(ref reference) => {
            println!("head_kind={}", "symbolic");
            println!("ref_full={}", reference.name);
            match reference.name.category_and_short_name() {
                Some((category, short_name)) => {
                    println!("ref_kind={:?}", category);
                    println!("ref_name={}", short_name);
                }
                None => {
                    println!("ref_kind=");
                    println!("ref_name=");
                }
            }

            // We care about reference target if it’s symbolic.
            match &reference.target {
                git_ref::Target::Peeled(oid) => {
                    println!("ref2_full={}", oid);
                    println!("ref2_kind=");
                    println!("ref2_name=");
                }
                git_ref::Target::Symbolic(full_name) => {
                    println!("ref2_full={}", full_name);
                    match full_name.category_and_short_name() {
                        Some((category, short_name)) => {
                            println!("ref2_kind={:?}", category);
                            println!("ref2_name={}", short_name);
                        }
                        None => {
                            println!("ref2_kind=");
                            println!("ref2_name=");
                        }
                    }
                }
            }
        },
        Kind::Unborn(ref full_name) => {
            println!("head_kind={}", "unborn");
            println!("ref_full={}", full_name);
            match full_name.category_and_short_name() {
                Some((category, short_name)) => {
                    println!("ref_kind={:?}", category);
                    println!("ref_name={}", short_name);
                }
                None => {
                    println!("ref_kind=");
                    println!("ref_name=");
                }
            }
            println!("ref2_full=");
            println!("ref2_kind=");
            println!("ref2_name=");
        },
        Kind::Detached{target: ref oid, ..} => {
            println!("head_kind={}", "detached");
            println!("ref_full={}", oid);
            println!("ref_kind=");
            println!("ref_name=");
            println!("ref2_full=");
            println!("ref2_kind=");
            println!("ref2_name=");
        },
    }

    match head.into_fully_peeled_id() {
        Some(Ok(id)) => {
            println!("ref_hash={}", id.detach());
        }
        Some(Err(error)) => {
            println!("ref_hash_error={:?}", error);
            println!("ref_hash=");
        }
        None => {
            println!("ref_hash=");
        }
    }

    Ok(())
}
