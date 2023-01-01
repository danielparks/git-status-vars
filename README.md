# Summarize git repo info into shell variables

This is designed to replace multiple calls to `git` with a single use of
`eval $(git-status-vars)`. It’s especially useful for generating a shell prompt.

This is intended to be generally usable in any theme that wants to report git
information in any shell with `sh`-like strings. I use it in my [personal ZSH
theme](https://github.com/danielparks/danielparks-zsh-theme).

## Installation

You can download binaries from the [GitHub releases page][releases]. Just
extract them and copy the file inside into your `$PATH`, e.g. `/usr/local/bin`.
The most common ones are:

  * [Linux on x86-64](https://github.com/danielparks/git-status-vars/releases/latest/download/git-status-vars-x86_64-unknown-linux-gnu.tar.gz)
  * [Linux on ARM](https://github.com/danielparks/git-status-vars/releases/latest/download/git-status-vars-aarch64-unknown-linux-musl.tar.gz)
  * [macOS on Intel](https://github.com/danielparks/git-status-vars/releases/latest/download/git-status-vars-x86_64-apple-darwin.tar.gz)
  * [macOS on Apple silicon](https://github.com/danielparks/git-status-vars/releases/latest/download/git-status-vars-aarch64-apple-darwin.tar.gz)
  * [Windows on x86-64](https://github.com/danielparks/git-status-vars/releases/latest/download/git-status-vars-x86_64-pc-windows-msvc.zip)

If you have `cargo`, you can use `cargo install`:

```sh
cargo install git-status-vars
```

[`cargo binstall`][binstall] also works.

## Usage

```sh
eval $(git-status-vars 2>/dev/null)
if [[ $repo_state == "NotFound" ]] ; then
  return 0
fi
```

This outputs a bunch of `sh` compatible environment variables about the current
repository. The repository is found by looking at each of the following in order
and taking the first that matches:

  1. Command line parameter. A repository directory, or a subdirectory of a
     repository, may be passed on the command line.
  2. The `$GIT_DIR` environment variable, just like `git`.
  3. A `.git` directory in the working directory or one of its parents.

`git-status-vars` will always output `repo_state=`, but all other variables may
be left out. In particular, if it can’t find a repository, it will output only
`repo_state=NotFound`.

### Example prompt function with `git-status-vars`

```sh
git_prompt () {
  eval $(git-status-vars 2>/dev/null)
  if [[ $repo_state == "NotFound" ]] ; then
    return 0
  fi

  local fg_color=green
  if (( $untracked_count > 0 )) ; then
    fg_color=red
  fi

  local ref=$head_ref1_short
  if [[ -z $ref ]] ; then
    ref=${head_hash:0:8}
  fi

  print -Pn "%F{$fg_color}${ref}%f "
}
```

### Equivalent prompt function without `git-status-vars`

```sh
git_prompt () {
  setopt local_options pipefail
  local untracked_count fg_color=green
  untracked_count=$(git ls-files --other --exclude-standard 2>/dev/null | wc -l)
  if (( $? != 0 )) ; then
    # No repository
    return 0
  fi

  local fg_color=green
  if (( $untracked_count > 0 )) ; then
    fg_color=red
  fi

  # Try for the branch or tag name, then try for the commit hash
  ref=$(git symbolic-ref --short HEAD 2>/dev/null) \
    || ref="$(git show-ref --head --hash --abbrev HEAD 2>/dev/null | head -n1)"

  print -Pn "%F{$fg_color}${ref}%f "
}
```

### Typical output

```
~/projects/git-status-vars ❯ git-status-vars
repo_state=Clean
repo_workdir=/Users/daniel/projects/git-status-vars/
repo_empty=false
repo_bare=false
head_ref_length=1
head_ref1_name=refs/heads/main
head_ref1_short=main
head_ref1_kind=direct
head_ref1_error=''
head_hash=2df6b768e60fbf899d8c8dc4a20385f30ee5da24
head_ahead=0
head_behind=0
head_upstream_error=''
untracked_count=0
unstaged_count=0
staged_count=0
conflicted_count=0
~/projects/git-status-vars ❯ cd /
/ ❯ git-status-vars
repo_state=NotFound
```

## Performance

`git-status-vars` is generally faster than multiple calls to `git`, though `git`
is fast enough that the difference will not usually be perceptible. On my laptop
`git-status-vars` typically runs in around 8 ms whereas the fallback code
involving multiple calls to `git` takes around 25 ms.

I have not tested this on large repositories.

## Rust Crate

[![docs.rs](https://img.shields.io/docsrs/git-status-vars)][docs.rs]
[![Crates.io](https://img.shields.io/crates/v/git-status-vars)][crates.io]

I’m not sure how useful it is, but this may be used from other Rust code.

## Change Log

Notable changes are tracked in [CHANGELOG.md](CHANGELOG.md). It is kept updated
with code changes, then a new “Release” header is added when a release is cut.
The changes from the release are added to the git tag and to the [release
description][releases] on GitHub.

## License

This project dual-licensed under the Apache 2 and MIT licenses. You may choose
to use either.

 * [Apache License, Version 2.0](LICENSE-APACHE)
 * [MIT license](LICENSE-MIT)

### Contributions

Unless you explicitly state otherwise, any contribution you submit as defined
in the Apache 2.0 license shall be dual licensed as above, without any
additional terms or conditions.

[binstall]: https://github.com/cargo-bins/cargo-binstall
[crates.io]: https://crates.io/crates/git-status-vars
[docs.rs]: https://docs.rs/git-status-vars/latest/git_status_vars/
[releases]: https://github.com/danielparks/git-status-vars/releases
