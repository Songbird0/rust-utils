
/// Displays a finest panic message by printing the value identificator (if any).
/// ## Panics
/// Panics if your integer isn't equal or greater than `boundary`.
/// ## Examples
///
/// ```rust,ignore
/// # #[macro_use] extern crate rustutils;
/// let foo = "123";
/// equal_or_greater!(foo.len(), 10); // will panic
/// equal_or_greater!(foo.len(), 3); // ok!
/// equal_or_greater!(foo.len(), 2); // ok!
/// ```
#[macro_export]
macro_rules! equal_or_greater {

    ($name: expr, $boundary: expr) => (
        if $name < $boundary {
            panic!("{0}({1}) must be equal or greater than {2}({3}).", 
            stringify!($name), 
            $name, 
            stringify!($boundary),
            $boundary);
        }
    );
}


/// Displays a finest panic message by printing the value identificator (if any).
/// ## Panics
/// Panics if your integer is negative or null.
/// ## Examples
///
/// ```rust,ignore
/// # #[macro_use] extern crate rustutils;
/// cannot_be_non!(0); // will panic
/// cannot_be_non!(-177); // will panic
/// cannot_be_non!(117); // ok!
/// ```
#[macro_export]
macro_rules! cannot_be_non {
    ($name:expr) => (
        if $name <= 0 {
            panic!("{0}({1}) cannot be negative or null.", stringify!($name), $name);
        }
    );
}
