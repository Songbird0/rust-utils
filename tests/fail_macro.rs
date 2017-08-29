#[macro_use]
extern crate rustutils;


#[test]
#[should_panic(expected = "The 'fail_panicked_test_1' unit test has not been implemented yet.")]
fn fail_panicked_test_1() {
    let foo = "fail_panicked_test_1";
    fail!(&foo);
}
