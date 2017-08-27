extern crate rustutils;

use rustutils::string::operations;
use std::error::Error;

#[test]
fn two_tags_successful_test_1() {
    let foo = match operations::two_tags("`hello`".to_owned(), ("[inline]", "[/inline]"), "`") {
        Ok(bbcode) => bbcode,
        Err(error) => {
            unreachable!(
                "Oops, an unexpected error was occurred!\nDebug:{}",
                error.description()
            )
        }
    };
    assert_eq!(foo, "[inline]hello[/inline]");
}
