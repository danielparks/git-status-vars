//! Test the test helpers.

#[expect(dead_code, reason = "helpers has code to support other modules")]
mod helpers;

#[test]
fn test_strip_indent_change() {
    assert_eq!(
        helpers::strip_indent(
            "
            No newline before this.
            Second line.
                Indented line.
            "
        ),
        "No newline before this.\nSecond line.\n    Indented line.\n",
    );
}

#[test]
fn test_strip_indent_unchanged() {
    let unchanged = "Doesn’t start with a newline
        This indent will not be removed.
        In fact, nothing will change.";
    assert_eq!(helpers::strip_indent(unchanged), unchanged);
}
