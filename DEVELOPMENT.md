# Development

## Change Log

Notable changes are tracked in [CHANGELOG.md](CHANGELOG.md). It is manually
updated when significant changes are made.

During the release process a new “Release” header is added and the changes from
the current release are added to the git tag and to the [release
description][releases] on GitHub.

## Versioning

This follows semantic versioning for the command line utility, not the crate
API. Breaking changes to the API are not guaranteed to involve a major version
change, since I don’t anticipate this being used as a crate by anyone else.

## Release process

 1. Run `./release.sh X.Y.Z` to update versions in code. If the diff looks good,
    run `git add . && git commit -m "Release X.Y.Z."`.
 2. Run `./release.sh X.Y.Z` again. It will tag the release and push it to
    GitHub. The new tag will trigger a job to build binaries, generate a GitHub
    release, and publish the package to crates.io.
 3. Confirm that the script made a “Prepping CHANGELOG.md for development.”
    commit and push it to GitHub.

## License

Unless otherwise noted, this project is dual-licensed under the Apache 2 and MIT
licenses. You may choose to use either.

 * [Apache License, Version 2.0](LICENSE-APACHE)
 * [MIT license](LICENSE-MIT)

### Contributions

Unless you explicitly state otherwise, any contribution you submit as defined
in the Apache 2.0 license shall be dual licensed as above, without any
additional terms or conditions.

[releases]: https://github.com/danielparks/git-status-vars/releases
