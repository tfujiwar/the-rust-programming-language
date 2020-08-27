extern crate adder;

#[test]
fn integration_test() {
    assert_eq!(4, adder::add_two(2));
}