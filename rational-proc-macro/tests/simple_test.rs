use rational::*;
use rational_proc_macro::*;

#[test]
fn simple_test() {
    let a = rational!(0);
    assert_eq!(a, Rational::from(0));

    let a = rational!(1.5);
    assert_eq!(a, Rational::new(3,2));
}