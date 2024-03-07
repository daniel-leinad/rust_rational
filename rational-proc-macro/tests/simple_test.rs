use rational::*;
use rational_proc_macro::*;

#[test]
fn simple_test() {
    let a = rational!(0);
    assert_eq!(a, Rational::from(0));

    let a = rational!(1.5);
    assert_eq!(a, Rational::new(3,2));

    let a = rational!(-23.57);
    assert_eq!(a, Rational::new(-2357, 100));

    let a = rational!(.15);
    assert_eq!(a, Rational::new(15, 100));

    let a = rational!(-0.);
    assert_eq!(a, Rational::new(0, 1));

    let a = rational!(-1.);
    assert_eq!(a, Rational::new(-1, 1));
}

#[test]
fn repeating() {
    let a = rational!(0.(3));
    assert_eq!(a, Rational::new(1, 3));

    let a = rational!(-0.(3));
    assert_eq!(a, Rational::new(-1, 3));

    let a = rational!(3.(571428));
    assert_eq!(a, Rational::new(25, 7));

    let a = rational!(-.(3));
    assert_eq!(a, Rational::new(-1, 3));

    let a = rational!(.(3));
    assert_eq!(a, Rational::new(1, 3));
}