#[macro_use]
extern crate rustutils;

#[test]
#[should_panic(expected = "0(0) cannot be negative or null.")]
fn cannot_be_non_panicked_test_1() {
    cannot_be_non!(0); // will panic
}

#[test]
#[should_panic(expected = "-177(-177) cannot be negative or null.")]
fn cannot_be_non_panicked_test_2() {
    cannot_be_non!(-177); // will panic
}

#[test]
#[should_panic(expected = "integer(-259) cannot be negative or null.")]
fn cannot_be_non_panicked_test_3() {
    let integer = -259;
    cannot_be_non!(integer);
}

#[test]
fn cannot_be_non_successfull_test_1() {
    cannot_be_non!(117); // ok!
}
