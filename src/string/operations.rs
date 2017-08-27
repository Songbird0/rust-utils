// Internal

use ::string::errors;

/// Wraps a sequence with a couple of tags.
/// ## Errors
/// Returns `Err()` if the supplied `delimiter` was not found, `Ok()` otherwise.
/// ## Panics
/// Panics if the `sentence` length isn't equal or greater than 2.
/// ## Examples
///
/// ```
/// use rustutils::string::operations;
/// let foo = operations::two_tags("`hello`".to_owned(), ("[inline]", "[/inline]"), "`").unwrap();
/// assert_eq!(foo, "[inline]hello[/inline]");
/// ```
pub fn two_tags(
    sentence: String,
    couple: (&str, &str),
    delimiter: &str,
) -> ::std::result::Result<String, errors::FirstElementNotFound> {
    let mut _sentence = sentence;
    equal_or_greater!(_sentence.len(), 2);
    let first_idx = match _sentence.find(delimiter) {
        Some(idx) => idx,
        None => return Err(errors::FirstElementNotFound::new(&delimiter)),
    };

    _sentence.remove(first_idx);
    // last removed element
    _sentence.pop();
    let mut wrapped_foo = String::new();
    let (opened_tag, closed_tag) = couple;
    wrapped_foo.push_str(opened_tag);
    wrapped_foo.push_str(_sentence.as_str());
    wrapped_foo.push_str(closed_tag);
    ::std::mem::drop(_sentence);
    Ok(wrapped_foo)
}
