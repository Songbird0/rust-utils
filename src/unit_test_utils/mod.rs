/*!
This module is dedicated to unit test tools. There are some basic macros to generate your unit tests functions and mark them as "unimplemented" with
intelligible error messages.
*/

/// Panics the unit test and prints an error message.
/// ```rust,ignore
/// # #[macro_use]
/// # extern crate rustutils;
/// let foo = "fail_panicked_test_1";
/// fail!(&foo); // prints: The 'fail_panicked_test_1' unit test has not been implemented yet.
/// ```
#[macro_export]
macro_rules! fail {
    ($unit_test_name:expr) => (
        panic!("The '{}' unit test has not been implemented yet.", &$unit_test_name)
    );
}
