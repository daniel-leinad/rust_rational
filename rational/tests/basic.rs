use rational::*;

#[test]
fn it_works() {
    let _result = Rational::new(15, 3);
}

#[test]
fn can_construct_zero() {
    let _res = Rational::new(0, 1);
    let _res = Rational::new(0, -1);
    let _res = Rational::new(0, 500);
}

#[should_panic(expected = "Denominator can't be zero!")]
#[test]
fn cant_make_denominator_zero() {
    let _result = Rational::new(1, 0);
}

#[test]
fn new_unchecked_works() {
    let _res = Rational::new_unchecked(10, 3);
}

#[should_panic(expected = "Denominator can't be zero!")]
#[test]
fn new_unchecked_does_not_work_when_denominator_is_zero() {
    let _res = Rational::new_unchecked(10, 0);
}
