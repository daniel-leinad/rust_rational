use rational::*;
use std::hash::Hasher;

#[test]
fn it_equates() {
    for equal_pair in equals_set() {
        assert_eq!(equal_pair.0, equal_pair.1, "pair: {equal_pair:?}");
    }
}

#[test]
fn equal_rationals_equal_hash() {
    for equal_pair in equals_set() {
        let hash_1 = default_hash(equal_pair.0);
        let hash_2 = default_hash(equal_pair.1);
        assert_eq!(hash_1, hash_2, "pair: {equal_pair:?}");
    }
}

#[test]
fn it_non_equates() {
    let a = Rational::new(15, 1);
    let b = Rational::new(3, 1);
    assert_ne!(a, b);

    let a = Rational::new(1, 1);
    let b = Rational::new(-1, 1);
    assert_ne!(a, b);

    let a = Rational::new(1, 14);
    let b = Rational::new(14, 1);
    assert_ne!(a, b);
}

#[test]
fn it_multiplies() {
    check_multiplication(0.into(), 5.into(), 0.into());
    check_multiplication(1.into(), 5.into(), 5.into());

    check_multiplication(3.into(), 5.into(), 15.into());
    check_multiplication(3.into(), (-5).into(), (-15).into());

    let a = Rational::new(15, 10);
    let b = Rational::new(6, 1);
    check_multiplication(a, b, 9.into());

    let a = Rational::new(-1, 2);
    let b: Rational = 3.into();
    check_multiplication(a, b, Rational::new(-3, 2));
}

#[test]
fn it_divides() {
    let a: Rational = 15.into();
    let b: Rational = 3.into();
    assert_eq!(a / b, 5.into());

    let a: Rational = (-15).into();
    let b: Rational = 3.into();
    assert_eq!(a / b, (-5).into());

    let a: Rational = 1.into();
    let b: Rational = 5.into();
    assert_eq!(a / b, Rational::new(1, 5));

    let a: Rational = (-3).into();
    let b: Rational = (-5).into();
    assert_eq!(a / b, Rational::new(3, 5));
}

#[should_panic(expected = "Can't divide by zero")]
#[test]
fn doesnt_divide_by_zero() {
    let a: Rational = 1.into();
    let b: Rational = 0.into();
    let _res = a / b;
}

#[test]
fn it_adds() {
    check_addition(1.into(), 2.into(), 3.into());
    check_addition(0.into(), 5.into(), 5.into());
    check_addition((-3).into(), 5.into(), 2.into());
    check_addition(
        "2.3".parse().unwrap(),
        "10.185".parse().unwrap(),
        "12.485".parse().unwrap(),
    );
}

#[test]
fn it_negates() {
    let a: Rational = 5.into();
    assert_eq!(-a, (-5).into());
}

#[test]
fn it_subtracts() {
    let a: Rational = 5.into();
    let b = 0.into();
    check_subtraction(a, b, 5.into());

    let a: Rational = 5.into();
    let b = 3.into();
    check_subtraction(a, b, 2.into());

    let a: Rational = 5.into();
    let b = (-3).into();
    check_subtraction(a, b, 8.into());

    let a: Rational = "0.3".parse().unwrap();
    let b = "0.1".parse().unwrap();
    check_subtraction(a, b, "0.2".parse().unwrap());
}

#[test]
fn it_compares() {
    let a: Rational = "0.5".parse().unwrap();
    let b: Rational = "0.6".parse().unwrap();

    assert!(b > a);
    assert!(b >= a);
    assert!(a < b);
    assert!(a <= b);

    let a: Rational = "0.5".parse().unwrap();
    let b: Rational = "20".parse().unwrap();

    assert!(b > a);
    assert!(b >= a);
    assert!(a < b);
    assert!(a <= b);

    let a: Rational = "0.5".parse().unwrap();
    let b: Rational = "-0.6".parse().unwrap();

    assert!(b < a);
    assert!(b <= a);
    assert!(a > b);
    assert!(a >= b);

    let a: Rational = "-0.5".parse().unwrap();
    let b: Rational = "20".parse().unwrap();

    assert!(b > a);
    assert!(b >= a);
    assert!(a < b);
    assert!(a <= b);

    let a: Rational = "-0.5".parse().unwrap();
    let b: Rational = "-0.6".parse().unwrap();

    assert!(b < a);
    assert!(b <= a);
    assert!(a > b);
    assert!(a >= b);

    let a: Rational = "-0.5".parse().unwrap();
    let b: Rational = "-20".parse().unwrap();

    assert!(b < a);
    assert!(b <= a);
    assert!(a > b);
    assert!(a >= b);

    let a: Rational = 5.into();
    let b: Rational = 6.into();

    assert!(a < b);
    assert!(a <= b);
    assert!(b > a);
    assert!(b >= a);

    let a = Rational::new(5, -1);
    let b = Rational::new(6, -1);

    assert!(a > b);
    assert!(a >= b);
    assert!(b < a);
    assert!(b <= a);

    let a = Rational::new(-5, 1);
    let b = Rational::new(6, -1);

    assert!(a > b);
    assert!(a >= b);
    assert!(b < a);
    assert!(b <= a);

    let a = Rational::new(5, -1);
    let b = Rational::new(-6, 1);

    assert!(a > b);
    assert!(a >= b);
    assert!(b < a);
    assert!(b <= a);

    let a = Rational::new(-5, 1);
    let b = Rational::new(-6, 1);

    assert!(a > b);
    assert!(a >= b);
    assert!(b < a);
    assert!(b <= a);
}

#[test]
fn it_compares_equals() {
    let a = Rational::new(5, 1);
    let b = Rational::new(5, 1);

    assert!(a >= b);
    assert!(b >= a);
    assert!(a <= b);
    assert!(b <= a);

    let a = Rational::new(-5, 1);
    let b = Rational::new(5, -1);

    assert!(a >= b);
    assert!(b >= a);
    assert!(a <= b);
    assert!(b <= a);

    let a = Rational::new(-5, -1);
    let b = Rational::new(5, 1);

    assert!(a >= b);
    assert!(b >= a);
    assert!(a <= b);
    assert!(b <= a);

    let a = Rational::new(5, 2);
    let b = Rational::new(5, 2);

    assert!(a >= b);
    assert!(b >= a);
    assert!(a <= b);
    assert!(b <= a);

    let a = Rational::new(-5, 2);
    let b = Rational::new(5, -2);

    assert!(a >= b);
    assert!(b >= a);
    assert!(a <= b);
    assert!(b <= a);

    let a = Rational::new(-5, -2);
    let b = Rational::new(5, 2);

    assert!(a >= b);
    assert!(b >= a);
    assert!(a <= b);
    assert!(b <= a);
}

fn check_addition(a: Rational, b: Rational, res: Rational) {
    assert_eq!(a + b, res);
    assert_eq!(b + a, res);
}

fn check_multiplication(a: Rational, b: Rational, res: Rational) {
    assert_eq!(a * b, res);
    assert_eq!(b * a, res);
}

fn check_subtraction(a: Rational, b: Rational, res: Rational) {
    assert_eq!(a - b, res);
    assert_eq!(b - a, -res);
}

fn equals_set() -> Vec<(Rational, Rational)> {
    let mut res = vec![];
    res.push((Rational::new(0, 1), Rational::new(0, -1343535)));
    res.push((Rational::new(15, 5), Rational::new(15, 5)));
    res.push((Rational::new(15, 5), Rational::new(3, 1)));
    res.push((Rational::new(15, -5), Rational::new(-15, 5)));
    res.push((Rational::new(15, 5), Rational::new(-3, -1)));
    res
}

fn default_hash<T: std::hash::Hash>(value: T) -> u64 {
    use std::hash::Hash;
    let mut hasher = std::hash::DefaultHasher::default();
    value.hash(&mut hasher);
    hasher.finish()
}
