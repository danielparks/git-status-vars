# Change log

All notable changes to this project will be documented in this file.

## main branch

## Release 0.2.4 (2023-01-21)

### Security fixes

* Upgrade [git2] dependency to 0.16.1 to fix a [security vulnerability in its
  handling of SSH keys][GHSA-m4ch-rfv5-x5g3]. This was unlikely to affect
  git-status-vars since it doesn’t fetch data from, or otherwise interact with,
  remote repositories.

[git2]: https://crates.io/crates/git2
[GHSA-m4ch-rfv5-x5g3]: https://github.com/rust-lang/git2-rs/security/advisories/GHSA-m4ch-rfv5-x5g3

## Release 0.2.3 (2022-12-31)

* Add download links to README.md.

## Release 0.2.2 (2022-12-31)

### Changes

* Bump version to test release workflow.

## Release 0.2.1 (2022-12-31)

### Changes

* Improvements to README.md and the overall crate documentation.

## Release 0.2.0 (2022-12-23)

### Features

* Added `repo_workdir` variable to output.
* Moved `summarize_repository()` and `summarize_opened_repository()` functions
  into `lib.rs`.

### Changes

* Added integration tests for various git repo states.
* Added Vagrant configuration for reproducible local tests on Linux.
* Added change log.
* Updated edition to Rust 2021.
