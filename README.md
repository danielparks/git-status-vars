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

  * Linux: [x86-64](https://github.com/danielparks/git-status-vars/releases/latest/download/git-status-vars-x86_64-unknown-linux-gnu.tar.gz),
    [ARM](https://github.com/danielparks/git-status-vars/releases/latest/download/git-status-vars-aarch64-unknown-linux-musl.tar.gz)
  * macOS: [Intel](https://github.com/danielparks/git-status-vars/releases/latest/download/git-status-vars-x86_64-apple-darwin.tar.gz),
    [Apple silicon](https://github.com/danielparks/git-status-vars/releases/latest/download/git-status-vars-aarch64-apple-darwin.tar.gz)
  * [Windows on x86-64](https://github.com/danielparks/git-status-vars/releases/latest/download/git-status-vars-x86_64-pc-windows-msvc.zip)

If you have `cargo`, you can just do `cargo install git-status-vars` to install
from source, or if you’ve installed [`cargo binstall`][binstall] you can use
that (`cargo binstall git-status-vars`).

[![Release status](https://github.com/danielparks/git-status-vars/actions/workflows/release.yaml/badge.svg)](https://github.com/danielparks/git-status-vars/actions/workflows/release.yaml)

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
stash_count=0
repo_state=Clean
~/projects/git-status-vars ❯ cd /
/ ❯ git-status-vars
repo_state=NotFound
```

## Performance

`git-status-vars` is generally faster than multiple calls to `git`, though `git`
is fast enough that the difference will not usually be perceptible. On my laptop
`git-status-vars` typically runs in around 8 ms whereas the fallback code
involving multiple calls to `git` takes around 25 ms.

By default, `git-status-vars` times out after 1 second. It will output as much
information about the repository as it can, and then it will output
`repo_state=Error`. Example output:

```
repo_workdir=/Users/daniel/personal/projects/git-status-vars/
repo_empty=false
repo_bare=false
head_ref_length=1
head_ref1_name=refs/heads/timeout
head_ref1_short=timeout
head_ref1_kind=direct
head_ref1_error=''
head_hash=768535e06fe7255908ea0b16c47b6b676b86b6af
head_ahead=2
head_behind=3
head_upstream_error=''
repo_state=Error
repo_error='Timed out'
```

I have not tested this on large repositories.

## Rust Crate

[![docs.rs](https://img.shields.io/docsrs/git-status-vars)][docs.rs]
[![Crates.io](https://img.shields.io/crates/v/git-status-vars)][crates.io]
![Rust version 1.64+](https://img.shields.io/badge/Rust%20version-1.64%2B-success)

I’m not sure how useful it is, but this may be used from other Rust code.

Currently the minimum supported Rust version (MSRV) is **1.74.1**.

## Development and contributions

See [DEVELOPMENT.md](DEVELOPMENT.md) for notes on contributing to this repo,
the license, how changes are tracked, and how releases are made.

### Development status

This is stable. I don’t have plans for additional features, but if you have
ideas please either submit an [issue][] or a [pull request][].

I will periodically update this to ensure that it doesn’t bit rot as
dependencies are updated, but you should not expect active development.

[binstall]: https://github.com/cargo-bins/cargo-binstall
[crates.io]: https://crates.io/crates/git-status-vars
[docs.rs]: https://docs.rs/git-status-vars/latest/git_status_vars/
[releases]: https://github.com/danielparks/git-status-vars/releases
[issue]: https://github.com/danielparks/git-status-vars/issues
[pull request]: https://github.com/danielparks/git-status-vars/pulls
