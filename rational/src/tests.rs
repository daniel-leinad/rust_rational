use super::*;

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