//! Test the test helpers.

#![forbid(unsafe_code)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::let_underscore_untyped,
    clippy::manual_string_new,
    clippy::map_unwrap_or,
    clippy::module_name_repetitions
)]
// Require docs on everything
#![warn(missing_docs, clippy::missing_docs_in_private_items)]
// Other restriction lints
#![warn(clippy::arithmetic_side_effects, clippy::dbg_macro)]

// We don’t use everything in helpers.
#[allow(dead_code)]
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
