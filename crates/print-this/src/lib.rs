//! Tool to print stuff.

use std::fmt::Display;

/// Transform to string.
pub fn to_string(this: impl Display) -> String {
    format!("{this}")
}

#[test]
fn prints_this() {
    let this = to_string("this");
    insta::assert_snapshot!(this, @"this");
}
