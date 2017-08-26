#[macro_export]
/// ## Panics
/// Panics if your string is empty.
/// ## Examples
///
/// ```rust,ignore
/// # #[macro_use] extern crate rustutils;
/// cannot_be_empty!(""); // will panic
/// cannot_be_empty!("".to_owned()); // will panic
/// cannot_be_empty!("Hello world!"); // ok!
/// ```
macro_rules! cannot_be_empty {
    ($name:expr) => (
        if $name.is_empty() {
            panic!("The '{}' string cannot be empty.", stringify!($name));
        }
    );
}
