use super::*;

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
fn it_reduces() {
    let result = Rational::new(1000, 1);
    assert_eq!(result.p.abs(), 1000);
    assert_eq!(result.q.abs(), 1);

    let result = Rational::new(1000, 2);
    assert_eq!(result.p.abs(), 500);
    assert_eq!(result.q.abs(), 1);

    let result = Rational::new(15, 3);
    assert_eq!(result.p.abs(), 5);
    assert_eq!(result.q.abs(), 1);

    let result = Rational::new(15, 6);
    assert_eq!(result.p.abs(), 5);
    assert_eq!(result.q.abs(), 2);

    let result = Rational::new(-15, 3);
    assert_eq!(result.p.abs(), 5);
    assert_eq!(result.q.abs(), 1);

    let result = Rational::new(15, -6);
    assert_eq!(result.p.abs(), 5);
    assert_eq!(result.q.abs(), 2);
}

#[test]
fn new_reduced_works() {
    let _res = Rational::new_reduced(10, 3); 
}

#[should_panic(expected = "Denominator can't be zero!")]
#[test]
fn new_reduced_does_not_work_when_denominator_is_zero() {
    let _res = Rational::new_reduced(10, 0);
}