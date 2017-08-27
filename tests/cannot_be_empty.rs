#[macro_use]
extern crate rustutils;

#[test]
#[should_panic(expected = "The 'foo' string cannot be empty.")]
fn cannot_be_empty_panicked_test_1() {
    let foo = "";
    cannot_be_empty!(foo);
}

#[test]
#[should_panic(expected = "The 'bar' string cannot be empty")]
fn cannot_be_empty_panicked_test_2() {
    let bar = "".to_owned();
    cannot_be_empty!(bar);
}

#[test]
fn cannot_be_empty_successful_test_1() {
    cannot_be_empty!("Hello world!");
}
