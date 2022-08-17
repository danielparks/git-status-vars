# Summarize git repo info into shell variables

![docs.rs](https://img.shields.io/docsrs/git-status-vars)
![Crates.io](https://img.shields.io/crates/v/git-status-vars)

This is designed to replace multiple calls to git with a single use of
`eval $(git-status-vars)`. It’s especially useful for generating a shell prompt.

It is generally faster than multiple calls to `git`, though `git` is fast enough
that the difference will not usually be perceptible. On my laptop
`git-status-vars` typically runs in around 8 ms whereas the fallback code
involving multiple calls to `git` takes around 25 ms.

This is intended to be generally usable in any theme that wants to report git
information in any shell with `sh`-like strings. I use it in my [personal ZSH
theme](https://github.com/danielparks/danielparks-zsh-theme).

### Example prompt without `git-status-vars`

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

### Example prompt with `git-status-vars`

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

### Typical output

```
~/projects/git-status-vars ❯ git-status-vars
repo_state=Clean
repo_empty=false
repo_bare=false
head_ref_length=1
head_ref1_name=refs/heads/main
head_ref1_short=main
head_ref1_kind=direct
head_ref1_error=''
head_hash=b175bd90e970a68bd108cf11d0b75a47c0134d4f
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

## License

This project dual-licensed under the Apache 2 and MIT licenses. You may choose
to use either.

 * [Apache License, Version 2.0](LICENSE-APACHE)
 * [MIT license](LICENSE-MIT)

### Contributions

Unless you explicitly state otherwise, any contribution you submit as defined
in the Apache 2.0 license shall be dual licensed as above, without any
additional terms or conditions.
