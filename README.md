# git-summary

Use to generate a prompt.

Currently this is targeted at returning the following:

```sh
git diff --quiet --ignore-submodules HEAD &>/dev/null
dirty=$?

untracked_files=$(git ls-files --other --exclude-standard 2>/dev/null | wc -l)

# Try for the branch or tag name, then try for the commit hash
ref=$(git symbolic-ref HEAD 2>/dev/null) \
  || ref="$(git show-ref --head --hash --abbrev HEAD | head -n1 2>/dev/null)"
```

### Example output

```
❯ target/release/git_summary
repo_state=Clean
repo_empty=false
repo_bare=false
head_ref_length=1
head_ref1_name=refs/heads/main
head_ref1_short=main
head_ref1_kind=direct
head_ref1_error=''
head_hash=6ea382037453bdbe6f514564c62ccc6d479dd551
untracked_count=0
unstaged_count=0
staged_count=0
conflicted_count=0
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
