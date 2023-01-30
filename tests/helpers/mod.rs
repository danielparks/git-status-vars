use assert_cmd::cargo::cargo_bin;
use bstr::{BString, ByteSlice};
use duct::cmd;
use pretty_assertions::assert_str_eq;
use regex::Regex;
use std::ffi::OsString;
use std::fs;
use std::path::Path;

/// Run the crate binary and return its output if successful.
pub fn git_status_vars<I, S>(root: &Path, args: I) -> BString
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    let executable = cargo_bin(env!("CARGO_PKG_NAME"));
    cmd(executable, args.into_iter())
        .dir(root)
        .env("HOME", root)
        .env("GIT_CONFIG_GLOBAL", root.join(".gitconfig"))
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .stderr_to_stdout()
        .stdout_capture()
        .run()
        .unwrap()
        .stdout
        .into()
}

/// Set up a call to `git` in the `repo` directory.
fn run_git<I, S>(root: &Path, repo: &str, args: I) -> duct::Expression
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    cmd("git", args.into_iter())
        .dir(root.join(repo))
        .env("HOME", root)
        .env("GIT_CONFIG_GLOBAL", root.join(".gitconfig"))
        .env("GIT_CONFIG_SYSTEM", "/dev/null")
        .stderr_to_stdout()
        .stdout_capture()
}

/// Run `git` in the `repo` directory and report errors.
pub fn git<I, S>(root: &Path, repo: &str, args: I) -> std::io::Result<()>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    let args: Vec<OsString> = args.into_iter().map(|arg| arg.into()).collect();
    let shell_args =
        shell_words::join(args.iter().map(|arg| arg.to_string_lossy()));

    println!("`git {shell_args}` in {:?}", root.join(repo));
    let output = run_git(root, repo, args).run()?;
    print!("{}", output.stdout.as_bstr());
    Ok(())
}

/// Prepare root directory of a test.
///
/// `user.name` and `user.email` must be set for commits to work in GitHub
/// actions. Having them set also helps to avoid confusing warnings, as do the
/// settings in `advice`.
///
/// If `init.defaultBranch` is not set, `git` gives a warning about the default
/// branch being subject to change, and if you explicitly set the initial branch
/// on `git init` to something other than the default branch, it will register
/// the repo as non-empty even if there are no commits. (I’m not sure if this is
/// a bug or not.)
pub fn prepare_root(root: &Path) {
    dbg!(git2::Version::get());
    fs::write(
        root.join(".gitconfig"),
        "[user]\n\
        name = Name\n\
        email = name@example.com\n\
        [init]\n\
        defaultBranch = main\n\
        [advice]\n\
        detachedHead = false\n\
        skippedCherryPicks = false\n",
    )
    .unwrap();
}

/// Create a git repository.
pub fn git_init(root: &Path, name: &str) {
    git(root, ".", ["init", name]).unwrap();
}

/// Make a commit with files a and b.
pub fn make_commit(root: &Path, repo: &str, n: u8) {
    fs::write(root.join(repo).join("a"), format!("{n}a")).unwrap();
    fs::write(root.join(repo).join("b"), format!("{n}b")).unwrap();
    git(root, repo, ["add", "a", "b"]).unwrap();
    git(root, repo, ["commit", "-m", &format!("commit {n}")]).unwrap();
}

/// Check the output of git-status-vars against a string.
///
/// This produces an easy to read diff when necessary.
///
/// To make the string legible and easy to copy and paste, the expected output
/// can be passed in a not-quite-literal format. If the first character is a
/// newline, then it and all the spaces following it will be removed, and that
/// many spaces will be removed from the beginning of all following lines.
///
/// Also, it will replace the string `@REPO@` with the repo path (`repo`).
///
/// Because git hashes are not the same from run to run, it will replace any
/// output matching `_hash=[0-9a-f]{40}` with `_hash=@HASH@`.
///
/// ```no_run
/// assert_git_status_vars(
///     &root,
///     "repo"
///     r#"
///     repo_state=Clean
///     repo_workdir=@REPO@/
///     repo_empty=false
///     repo_bare=false
///     head_ref_length=1
///     head_ref1_name=refs/heads/main
///     head_ref1_short=main
///     head_ref1_kind=direct
///     head_ref1_error=''
///     head_hash=@HASH@
///     . . .
///     conflicted_count=0
///     "#,
/// );
/// ```
pub fn assert_git_status_vars(root: &Path, repo: &str, expected: &str) {
    // Strip first newline and indent
    let expected = if expected.bytes().next() == Some(b'\n') {
        if let Some(i) = expected[1..].find(|c: char| c != ' ') {
            // We skipped the first newline, so add 1 to the result.
            expected[i + 1..].replace(&expected[..i + 1], "\n")
        } else {
            // Didn’t find non-space character.
            String::from("")
        }
    } else {
        expected.to_string()
    };

    let re = Regex::new(r"_hash=[0-9a-f]{40}").unwrap();
    let output = git_status_vars(root, [repo]);
    let output = output.to_str_lossy();
    let output = re.replace_all(&output, "_hash=@HASH@");

    assert_str_eq!(
        expected.replace("@REPO@", &root.join(repo).display().to_string()),
        output,
    );
}
