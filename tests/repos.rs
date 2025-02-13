//! Tests results on various example repos.

use std::fs;
use target_test_dir::with_test_dir;

mod helpers;

#[test]
#[with_test_dir]
fn nonexistent() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);
    assert_eq!(
        "repo_state=NotFound\n",
        helpers::git_status_vars(&root, ["."])
    );
}

#[test]
#[with_test_dir]
fn empty() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "repo");

    helpers::assert_git_status_vars(
        &root,
        "repo",
        r#"
        repo_state=Clean
        repo_workdir=@REPO@/
        repo_empty=true
        repo_bare=false
        head_ref_length=1
        head_ref1_name=refs/heads/main
        head_ref1_short=main
        head_ref1_kind=''
        head_ref1_error='Error { code: -3, klass: 4, message: "reference '\''refs/heads/main'\'' not found" }'
        head_hash=''
        head_ahead=''
        head_behind=''
        head_upstream_error='Error { code: -9, klass: 4, message: "reference '\''refs/heads/main'\'' not found" }'
        untracked_count=0
        unstaged_count=0
        staged_count=0
        conflicted_count=0
        stash_count=0
        "#,
    );
}

#[test]
#[with_test_dir]
fn empty_untracked() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "repo");
    fs::write(root.join("repo").join("untracked"), "").unwrap();

    helpers::assert_git_status_vars(
        &root,
        "repo",
        r#"
        repo_state=Clean
        repo_workdir=@REPO@/
        repo_empty=true
        repo_bare=false
        head_ref_length=1
        head_ref1_name=refs/heads/main
        head_ref1_short=main
        head_ref1_kind=''
        head_ref1_error='Error { code: -3, klass: 4, message: "reference '\''refs/heads/main'\'' not found" }'
        head_hash=''
        head_ahead=''
        head_behind=''
        head_upstream_error='Error { code: -9, klass: 4, message: "reference '\''refs/heads/main'\'' not found" }'
        untracked_count=1
        unstaged_count=0
        staged_count=0
        conflicted_count=0
        stash_count=0
        "#,
    );
}

#[test]
#[with_test_dir]
fn empty_added() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "repo");
    fs::write(root.join("repo").join("added"), "").unwrap();
    helpers::git(&root, "repo", ["add", "added"]).unwrap();

    helpers::assert_git_status_vars(
        &root,
        "repo",
        r#"
        repo_state=Clean
        repo_workdir=@REPO@/
        repo_empty=true
        repo_bare=false
        head_ref_length=1
        head_ref1_name=refs/heads/main
        head_ref1_short=main
        head_ref1_kind=''
        head_ref1_error='Error { code: -3, klass: 4, message: "reference '\''refs/heads/main'\'' not found" }'
        head_hash=''
        head_ahead=''
        head_behind=''
        head_upstream_error='Error { code: -9, klass: 4, message: "reference '\''refs/heads/main'\'' not found" }'
        untracked_count=0
        unstaged_count=0
        staged_count=1
        conflicted_count=0
        stash_count=0
        "#,
    );
}

#[test]
#[with_test_dir]
fn empty_untracked_added() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "repo");
    fs::write(root.join("repo").join("added"), "").unwrap();
    fs::write(root.join("repo").join("untracked"), "").unwrap();
    helpers::git(&root, "repo", ["add", "added"]).unwrap();

    helpers::assert_git_status_vars(
        &root,
        "repo",
        r#"
        repo_state=Clean
        repo_workdir=@REPO@/
        repo_empty=true
        repo_bare=false
        head_ref_length=1
        head_ref1_name=refs/heads/main
        head_ref1_short=main
        head_ref1_kind=''
        head_ref1_error='Error { code: -3, klass: 4, message: "reference '\''refs/heads/main'\'' not found" }'
        head_hash=''
        head_ahead=''
        head_behind=''
        head_upstream_error='Error { code: -9, klass: 4, message: "reference '\''refs/heads/main'\'' not found" }'
        untracked_count=1
        unstaged_count=0
        staged_count=1
        conflicted_count=0
        stash_count=0
        "#,
    );
}

#[test]
#[with_test_dir]
fn commit() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "repo");
    helpers::make_commit(&root, "repo", 1);

    helpers::assert_git_status_vars(
        &root,
        "repo",
        r#"
        repo_state=Clean
        repo_workdir=@REPO@/
        repo_empty=false
        repo_bare=false
        head_ref_length=1
        head_ref1_name=refs/heads/main
        head_ref1_short=main
        head_ref1_kind=direct
        head_ref1_error=''
        head_hash=@HASH@
        head_ahead=''
        head_behind=''
        head_upstream_error='Error { code: -3, klass: 7, message: "config value '\''branch.main.remote'\'' was not found" }'
        untracked_count=0
        unstaged_count=0
        staged_count=0
        conflicted_count=0
        stash_count=0
        "#,
    );
}

#[test]
#[with_test_dir]
fn commit_delete() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "repo");
    helpers::make_commit(&root, "repo", 1);
    fs::remove_file(root.join("repo").join("a")).unwrap();

    helpers::assert_git_status_vars(
        &root,
        "repo",
        r#"
        repo_state=Clean
        repo_workdir=@REPO@/
        repo_empty=false
        repo_bare=false
        head_ref_length=1
        head_ref1_name=refs/heads/main
        head_ref1_short=main
        head_ref1_kind=direct
        head_ref1_error=''
        head_hash=@HASH@
        head_ahead=''
        head_behind=''
        head_upstream_error='Error { code: -3, klass: 7, message: "config value '\''branch.main.remote'\'' was not found" }'
        untracked_count=0
        unstaged_count=1
        staged_count=0
        conflicted_count=0
        stash_count=0
        "#,
    );
}

#[test]
#[with_test_dir]
fn commit_delete_staged() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "repo");
    helpers::make_commit(&root, "repo", 1);
    helpers::git(&root, "repo", ["rm", "a"]).unwrap();

    helpers::assert_git_status_vars(
        &root,
        "repo",
        r#"
        repo_state=Clean
        repo_workdir=@REPO@/
        repo_empty=false
        repo_bare=false
        head_ref_length=1
        head_ref1_name=refs/heads/main
        head_ref1_short=main
        head_ref1_kind=direct
        head_ref1_error=''
        head_hash=@HASH@
        head_ahead=''
        head_behind=''
        head_upstream_error='Error { code: -3, klass: 7, message: "config value '\''branch.main.remote'\'' was not found" }'
        untracked_count=0
        unstaged_count=0
        staged_count=1
        conflicted_count=0
        stash_count=0
        "#,
    );
}

#[test]
#[with_test_dir]
fn commit_modified() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "repo");
    helpers::make_commit(&root, "repo", 1);
    fs::write(root.join("repo").join("a"), "2a").unwrap();

    helpers::assert_git_status_vars(
        &root,
        "repo",
        r#"
        repo_state=Clean
        repo_workdir=@REPO@/
        repo_empty=false
        repo_bare=false
        head_ref_length=1
        head_ref1_name=refs/heads/main
        head_ref1_short=main
        head_ref1_kind=direct
        head_ref1_error=''
        head_hash=@HASH@
        head_ahead=''
        head_behind=''
        head_upstream_error='Error { code: -3, klass: 7, message: "config value '\''branch.main.remote'\'' was not found" }'
        untracked_count=0
        unstaged_count=1
        staged_count=0
        conflicted_count=0
        stash_count=0
        "#,
    );
}

#[test]
#[with_test_dir]
fn commit_modified_staged() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "repo");
    helpers::make_commit(&root, "repo", 1);
    fs::write(root.join("repo").join("a"), "2a").unwrap();
    helpers::git(&root, "repo", ["add", "a"]).unwrap();

    helpers::assert_git_status_vars(
        &root,
        "repo",
        r#"
        repo_state=Clean
        repo_workdir=@REPO@/
        repo_empty=false
        repo_bare=false
        head_ref_length=1
        head_ref1_name=refs/heads/main
        head_ref1_short=main
        head_ref1_kind=direct
        head_ref1_error=''
        head_hash=@HASH@
        head_ahead=''
        head_behind=''
        head_upstream_error='Error { code: -3, klass: 7, message: "config value '\''branch.main.remote'\'' was not found" }'
        untracked_count=0
        unstaged_count=0
        staged_count=1
        conflicted_count=0
        stash_count=0
        "#,
    );
}

#[test]
#[with_test_dir]
fn detached() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "repo");
    helpers::make_commit(&root, "repo", 1);
    helpers::make_commit(&root, "repo", 2);
    helpers::git(&root, "repo", ["checkout", "HEAD^"]).unwrap();

    helpers::assert_git_status_vars(
        &root,
        "repo",
        r#"
        repo_state=Clean
        repo_workdir=@REPO@/
        repo_empty=false
        repo_bare=false
        head_ref_length=0
        head_hash=@HASH@
        head_ahead=''
        head_behind=''
        head_upstream_error='Error { code: -1, klass: 3, message: "reference '\''HEAD'\'' is not a local branch." }'
        untracked_count=0
        unstaged_count=0
        staged_count=0
        conflicted_count=0
        stash_count=0
        "#,
    );
}

#[test]
#[with_test_dir]
fn branch() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "repo");
    helpers::make_commit(&root, "repo", 1);
    helpers::git(&root, "repo", ["switch", "-c", "branch"]).unwrap();

    helpers::assert_git_status_vars(
        &root,
        "repo",
        r#"
        repo_state=Clean
        repo_workdir=@REPO@/
        repo_empty=false
        repo_bare=false
        head_ref_length=1
        head_ref1_name=refs/heads/branch
        head_ref1_short=branch
        head_ref1_kind=direct
        head_ref1_error=''
        head_hash=@HASH@
        head_ahead=''
        head_behind=''
        head_upstream_error='Error { code: -3, klass: 7, message: "config value '\''branch.branch.remote'\'' was not found" }'
        untracked_count=0
        unstaged_count=0
        staged_count=0
        conflicted_count=0
        stash_count=0
        "#,
    );
}

#[test]
#[with_test_dir]
fn sym_ref() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "repo");
    helpers::make_commit(&root, "repo", 1);
    helpers::git(
        &root,
        "repo",
        ["symbolic-ref", "refs/heads/sym", "refs/heads/main"],
    )
    .unwrap();
    helpers::git(&root, "repo", ["switch", "sym"]).unwrap();

    helpers::assert_git_status_vars(
        &root,
        "repo",
        r#"
        repo_state=Clean
        repo_workdir=@REPO@/
        repo_empty=false
        repo_bare=false
        head_ref_length=2
        head_ref1_name=refs/heads/sym
        head_ref1_short=sym
        head_ref1_kind=symbolic
        head_ref1_error=''
        head_ref2_name=refs/heads/main
        head_ref2_short=main
        head_ref2_kind=direct
        head_ref2_error=''
        head_hash=@HASH@
        head_ahead=''
        head_behind=''
        head_upstream_error='Error { code: -3, klass: 7, message: "config value '\''branch.main.remote'\'' was not found" }'
        untracked_count=0
        unstaged_count=0
        staged_count=0
        conflicted_count=0
        stash_count=0
        "#,
    );
}

// Tags are actually just a detached HEAD. Including because why not.
#[test]
#[with_test_dir]
fn tag() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "repo");
    helpers::make_commit(&root, "repo", 1);
    helpers::make_commit(&root, "repo", 2);
    helpers::git(&root, "repo", ["tag", "tag-a", "HEAD^"]).unwrap();
    helpers::git(&root, "repo", ["checkout", "tag-a"]).unwrap();

    helpers::assert_git_status_vars(
        &root,
        "repo",
        r#"
        repo_state=Clean
        repo_workdir=@REPO@/
        repo_empty=false
        repo_bare=false
        head_ref_length=0
        head_hash=@HASH@
        head_ahead=''
        head_behind=''
        head_upstream_error='Error { code: -1, klass: 3, message: "reference '\''HEAD'\'' is not a local branch." }'
        untracked_count=0
        unstaged_count=0
        staged_count=0
        conflicted_count=0
        stash_count=0
        "#,
    );
}

#[test]
#[with_test_dir]
fn cherry_pick() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "repo");
    helpers::make_commit(&root, "repo", 1);
    helpers::git(&root, "repo", ["switch", "-c", "branch"]).unwrap();
    helpers::make_commit(&root, "repo", 2);
    helpers::git(&root, "repo", ["switch", "main"]).unwrap();
    helpers::make_commit(&root, "repo", 3);
    helpers::git(&root, "repo", ["cherry-pick", "branch"])
        .expect_err("cherry-pick should fail");

    helpers::assert_git_status_vars(
        &root,
        "repo",
        r#"
        repo_state=CherryPick
        repo_workdir=@REPO@/
        repo_empty=false
        repo_bare=false
        head_ref_length=1
        head_ref1_name=refs/heads/main
        head_ref1_short=main
        head_ref1_kind=direct
        head_ref1_error=''
        head_hash=@HASH@
        head_ahead=''
        head_behind=''
        head_upstream_error='Error { code: -3, klass: 7, message: "config value '\''branch.main.remote'\'' was not found" }'
        untracked_count=0
        unstaged_count=0
        staged_count=0
        conflicted_count=2
        stash_count=0
        "#,
    );
}

#[test]
#[with_test_dir]
fn cherry_pick_staged() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "repo");
    helpers::make_commit(&root, "repo", 1);
    helpers::git(&root, "repo", ["switch", "-c", "branch"]).unwrap();
    helpers::make_commit(&root, "repo", 2);
    helpers::git(&root, "repo", ["switch", "main"]).unwrap();
    helpers::make_commit(&root, "repo", 3);
    helpers::git(&root, "repo", ["cherry-pick", "branch"])
        .expect_err("cherry-pick should fail");
    fs::write(root.join("repo").join("a"), "4a").unwrap();
    helpers::git(&root, "repo", ["add", "a"]).unwrap();

    helpers::assert_git_status_vars(
        &root,
        "repo",
        r#"
        repo_state=CherryPick
        repo_workdir=@REPO@/
        repo_empty=false
        repo_bare=false
        head_ref_length=1
        head_ref1_name=refs/heads/main
        head_ref1_short=main
        head_ref1_kind=direct
        head_ref1_error=''
        head_hash=@HASH@
        head_ahead=''
        head_behind=''
        head_upstream_error='Error { code: -3, klass: 7, message: "config value '\''branch.main.remote'\'' was not found" }'
        untracked_count=0
        unstaged_count=0
        staged_count=1
        conflicted_count=1
        stash_count=0
        "#,
    );
}

#[test]
#[with_test_dir]
fn cherry_pick_unstaged() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "repo");
    helpers::make_commit(&root, "repo", 1);
    helpers::git(&root, "repo", ["switch", "-c", "branch"]).unwrap();
    helpers::make_commit(&root, "repo", 2);
    helpers::git(&root, "repo", ["switch", "main"]).unwrap();
    helpers::make_commit(&root, "repo", 3);
    helpers::git(&root, "repo", ["cherry-pick", "branch"])
        .expect_err("cherry-pick should fail");
    fs::write(root.join("repo").join("a"), "4a").unwrap();
    helpers::git(&root, "repo", ["add", "a"]).unwrap();
    helpers::git(&root, "repo", ["restore", "--staged", "a"]).unwrap();

    helpers::assert_git_status_vars(
        &root,
        "repo",
        r#"
        repo_state=CherryPick
        repo_workdir=@REPO@/
        repo_empty=false
        repo_bare=false
        head_ref_length=1
        head_ref1_name=refs/heads/main
        head_ref1_short=main
        head_ref1_kind=direct
        head_ref1_error=''
        head_hash=@HASH@
        head_ahead=''
        head_behind=''
        head_upstream_error='Error { code: -3, klass: 7, message: "config value '\''branch.main.remote'\'' was not found" }'
        untracked_count=0
        unstaged_count=1
        staged_count=0
        conflicted_count=1
        stash_count=0
        "#,
    );
}

#[test]
#[with_test_dir]
fn conflict() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "repo");
    helpers::make_commit(&root, "repo", 1);
    helpers::git(&root, "repo", ["switch", "-c", "branch"]).unwrap();
    helpers::make_commit(&root, "repo", 2);
    helpers::git(&root, "repo", ["switch", "main"]).unwrap();
    helpers::make_commit(&root, "repo", 3);
    helpers::git(&root, "repo", ["merge", "branch"])
        .expect_err("merge should fail");

    helpers::assert_git_status_vars(
        &root,
        "repo",
        r#"
        repo_state=Merge
        repo_workdir=@REPO@/
        repo_empty=false
        repo_bare=false
        head_ref_length=1
        head_ref1_name=refs/heads/main
        head_ref1_short=main
        head_ref1_kind=direct
        head_ref1_error=''
        head_hash=@HASH@
        head_ahead=''
        head_behind=''
        head_upstream_error='Error { code: -3, klass: 7, message: "config value '\''branch.main.remote'\'' was not found" }'
        untracked_count=0
        unstaged_count=0
        staged_count=0
        conflicted_count=2
        stash_count=0
        "#,
    );
}

#[test]
#[with_test_dir]
fn bare() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "upstream");
    helpers::make_commit(&root, "upstream", 1);
    helpers::git(&root, ".", ["clone", "--bare", "upstream", "bare"]).unwrap();

    helpers::assert_git_status_vars(
        &root,
        "bare",
        r#"
        repo_state=Clean
        repo_workdir=''
        repo_empty=false
        repo_bare=true
        head_ref_length=1
        head_ref1_name=refs/heads/main
        head_ref1_short=main
        head_ref1_kind=direct
        head_ref1_error=''
        head_hash=@HASH@
        head_ahead=''
        head_behind=''
        head_upstream_error='Error { code: -3, klass: 7, message: "config value '\''branch.main.remote'\'' was not found" }'
        untracked_count=0
        unstaged_count=0
        staged_count=0
        conflicted_count=0
        stash_count=0
        "#,
    );
}

#[test]
#[with_test_dir]
fn ahead_1() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "upstream");
    helpers::make_commit(&root, "upstream", 1);
    helpers::git(&root, ".", ["clone", "upstream", "clone"]).unwrap();
    helpers::make_commit(&root, "clone", 2);

    helpers::assert_git_status_vars(
        &root,
        "clone",
        r"
        repo_state=Clean
        repo_workdir=@REPO@/
        repo_empty=false
        repo_bare=false
        head_ref_length=1
        head_ref1_name=refs/heads/main
        head_ref1_short=main
        head_ref1_kind=direct
        head_ref1_error=''
        head_hash=@HASH@
        head_ahead=1
        head_behind=0
        head_upstream_error=''
        untracked_count=0
        unstaged_count=0
        staged_count=0
        conflicted_count=0
        stash_count=0
        ",
    );
}

#[test]
#[with_test_dir]
fn ahead_1_behind_1() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "upstream");
    helpers::make_commit(&root, "upstream", 1);
    helpers::git(&root, ".", ["clone", "upstream", "clone"]).unwrap();
    helpers::make_commit(&root, "upstream", 2);
    helpers::make_commit(&root, "clone", 3);
    helpers::git(&root, "clone", ["fetch"]).unwrap();

    helpers::assert_git_status_vars(
        &root,
        "clone",
        r"
        repo_state=Clean
        repo_workdir=@REPO@/
        repo_empty=false
        repo_bare=false
        head_ref_length=1
        head_ref1_name=refs/heads/main
        head_ref1_short=main
        head_ref1_kind=direct
        head_ref1_error=''
        head_hash=@HASH@
        head_ahead=1
        head_behind=1
        head_upstream_error=''
        untracked_count=0
        unstaged_count=0
        staged_count=0
        conflicted_count=0
        stash_count=0
        ",
    );
}

#[test]
#[with_test_dir]
fn behind_1() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "upstream");
    helpers::make_commit(&root, "upstream", 1);
    helpers::git(&root, ".", ["clone", "upstream", "clone"]).unwrap();
    helpers::make_commit(&root, "upstream", 2);
    helpers::git(&root, "clone", ["fetch"]).unwrap();

    helpers::assert_git_status_vars(
        &root,
        "clone",
        r"
        repo_state=Clean
        repo_workdir=@REPO@/
        repo_empty=false
        repo_bare=false
        head_ref_length=1
        head_ref1_name=refs/heads/main
        head_ref1_short=main
        head_ref1_kind=direct
        head_ref1_error=''
        head_hash=@HASH@
        head_ahead=0
        head_behind=1
        head_upstream_error=''
        untracked_count=0
        unstaged_count=0
        staged_count=0
        conflicted_count=0
        stash_count=0
        ",
    );
}

#[test]
#[with_test_dir]
fn stashed_1() {
    let root = get_test_dir!();
    helpers::prepare_root(&root);

    helpers::git_init(&root, "repo");
    helpers::make_commit(&root, "repo", 1);
    fs::write(root.join("repo").join("a"), "2a").unwrap();
    helpers::git(&root, "repo", ["stash", "push", "a"]).unwrap();

    helpers::assert_git_status_vars(
        &root,
        "repo",
        r#"
        repo_state=Clean
        repo_workdir=@REPO@/
        repo_empty=false
        repo_bare=false
        head_ref_length=1
        head_ref1_name=refs/heads/main
        head_ref1_short=main
        head_ref1_kind=direct
        head_ref1_error=''
        head_hash=@HASH@
        head_ahead=''
        head_behind=''
        head_upstream_error='Error { code: -3, klass: 7, message: "config value '\''branch.main.remote'\'' was not found" }'
        untracked_count=0
        unstaged_count=0
        staged_count=0
        conflicted_count=0
        stash_count=1
        "#,
    );
}
