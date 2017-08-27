#[macro_use]
extern crate rustutils;

#[test]
#[should_panic(expected = "foo.len()(3) must be equal or greater than 10(10)")]
fn equal_or_greater_panicked_test_1() {
    let foo = "123";
    equal_or_greater!(foo.len(), 10); // will panic
}

#[test]
fn equal_or_greater_successful_test_1() {
    let foo = "123";
    equal_or_greater!(foo.len(), 3); // ok!
}

#[test]
fn equal_or_greater_successful_test_2() {
    let foo = "123";
    equal_or_greater!(foo.len(), 2); // ok!
}
