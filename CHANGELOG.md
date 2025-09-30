# Change log

All notable changes to this project will be documented in this file.

## main branch

* Update `--timeout` to support sub-second timeouts; change the default timeout
  to 500ms.

## Release 1.2.0 (2025-08-01)

* Add `--timeout` option, which defaults to 1 second. `git-status-vars` will now
  output as much repository information as it can, then end with:

      repo_state=Error
      repo_error='Timed out'

## Release 1.1.1 (2025-02-12)

* Bump version to fix release workflow.

## Release 1.1.0 (2025-02-12)

* Add `stash_count` variable to output containing the number of stashed changes.

### API breaking changes

* `summarize_opened_repository()` now takes a `mut` reference to a `Repository`.
  This is necessary to count the number of stashed changes.

## Release 1.0.4 (2024-12-05)

### Security fixes

* Upgrade indirect [anstream] dependency (used by [clap]) to fix [a security
  vulnerability][GHSA-2rxc-gjrp-vjhx]. I am unsure if this affects
  git-status-vars. This means the MSRV is now 1.74.1.

[anstream]: https://crates.io/crates/anstream
[clap]: https://crates.io/crates/clap
[GHSA-2rxc-gjrp-vjhx]: https://github.com/advisories/GHSA-2rxc-gjrp-vjhx

## Release 1.0.3 (2024-02-12)

### Security fixes

* Upgrade [git2] dependency to 0.18.2 to fix [security vulnerabilities in
  libgit2][GHSA-22q8-ghmq-63vf], including in revision parsing. These do not
  appear to affect git-status-vars.

[git2]: https://crates.io/crates/git2
[GHSA-22q8-ghmq-63vf]: https://github.com/advisories/GHSA-22q8-ghmq-63vf

### API breaking changes

* Switched `Reference::new()` and friends to accept types that implement
  `std::fmt::Display` instead of `AsRef<str>`. The functions convert the
  parameters to owned `String`s with `to_string()` anyway, so this more
  accurately reflects what the functions are doing.

### Other API changes

* `ShellWriter::with_prefix()` now accepts anything that implements
  `std::fmt::Display` as the prefix rather than just `String`s.

## Release 1.0.2 (2023-05-24)

* Update documentation to reflect that the minimum supported Rust version
  (MSRV) is 1.64.

## Release 1.0.1 (2023-05-24)

* Added missing crate documentation.
* Updated dependencies.

### Bug fixes

* Ensured that `summarize_opened_repository()` would not produce output before
  returning an error. This could have caused confusing output from
  `summarize_repository()`.

## Release 1.0.0 (2023-04-05)

Bumping to version 1.0.0 to indicate stability. There are no functional changes.

* Update all dependencies.
* Update the minimum supported Rust version (MSRV) from 1.60 to 1.64.
* Document development status — stable; no more planned features.
* Document versioning policy — the version primarily tracks changes to the
  binary, not the crate as a library.

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
