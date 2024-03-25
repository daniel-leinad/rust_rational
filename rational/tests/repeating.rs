use rational::*;

#[test]
fn it_parses_string() {
    let a: Rational = "0.(3)".parse().unwrap();
    let b = Rational::new(1, 3);
    let c: Rational = "0.(333)".parse().unwrap();
    assert_eq!(a, b);
    assert_eq!(a, c);
}

#[test]
fn it_parses_negative_string() {
    let a: Rational = "-0.(3)".parse().unwrap();
    let b = Rational::new(-1, 3);
    assert_eq!(a, b);

    let a: Rational = "-0.(33)".parse().unwrap();
    let b = Rational::new(-1, 3);
    assert_eq!(a, b);
}

#[test]
fn it_parses_all_kinds_of_strings() {
    let a: Rational = "-0.0(3)".parse().unwrap();
    let b = Rational::new(-1, 30);
    assert_eq!(a, b);

    let a: Rational = "3.(571428)".parse().unwrap();
    let b = Rational::new(25, 7);
    assert_eq!(a, b);

    let a: Rational = "-3.(571428)".parse().unwrap();
    let b = Rational::new(-25, 7);
    assert_eq!(a, b);

    let a: Rational = "0.0(0)".parse().unwrap();
    let b = Rational::new(0, 1);
    assert_eq!(a, b);

    let a: Rational = "-0.0(0)".parse().unwrap();
    let b = Rational::new(0, 1);
    assert_eq!(a, b);

    let a: Rational = ".(3)".parse().unwrap();
    let b = Rational::new(1, 3);
    assert_eq!(a, b);

    let a: Rational = "-.(3)".parse().unwrap();
    let b = Rational::new(-1, 3);
    assert_eq!(a, b);

    let a: Rational = "0.0(410256)".parse().unwrap();
    let b = Rational::new(8, 195);
    assert_eq!(a, b);
}

#[test]
fn doesnt_parse_empty_repeating() {
    let a: Result<Rational, _> = "1.34()".parse();
    assert_eq!(a, Err("Error parsing string"));
}

#[test]
fn doesnt_parse_digits_after_repeating() {
    let a: Result<Rational, _> = "1.34(3)12".parse();
    assert_eq!(a, Err("Error parsing string"));
}

#[test]
fn doesnt_parse_anything_after_repeating() {
    let a: Result<Rational, _> = "1.34(3)--3".parse();
    assert_eq!(a, Err("Error parsing string"));
}

#[test]
fn doesnt_parse_incorrect_repeating_part() {
    let a: Result<Rational, _> = "1.34(3.2)".parse();
    assert_eq!(a, Err("Error parsing string"));
}

#[test]
fn doesnt_parse_all_kinds_of_incorrect_strings() {
    let a: Result<Rational, _> = "0(3)".parse();
    assert_eq!(a, Err("Error parsing string"));

    let a: Result<Rational, _> = "0(3).5".parse();
    assert_eq!(a, Err("Error parsing string"));

    let a: Result<Rational, _> = "0.(3".parse();
    assert_eq!(a, Err("Error parsing string"));

    let a: Result<Rational, _> = "0.3)".parse();
    assert_eq!(a, Err("Error parsing string"));
}

#[test]
fn doesnt_parse_no_digits_number() {
    let a: Result<Rational, _> = ".()".parse();
    assert_eq!(a, Err("Error parsing string"));
}