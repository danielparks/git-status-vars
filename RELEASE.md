# Release process

1. Create a release branch “release-vX.Y.Z” and submit as a PR.
  1. Update version in Cargo.toml and Cargo.lock.
  2. Replace "## main branch" with "## Release {{version}} ({{date}})" in
     CHANGELOG.md.
  3. Commit with message: "Release {{version}}."
2. Create release tag “vX.Y.Z”.
  1. Get the release notes from CHANGELOG.md, set as the tag body.
  2. Push tag to origin. The release.yaml workflow will run on GitHub, publish
     the package to crates.io, and build and publish binaries.
3. Post-release commit:
  1. Insert "## main branch" above first "## Release" in CHANGELOG.md.
  2. Commit with message: "Prepping CHANGELOG.md for development."
  3. Push to origin.
