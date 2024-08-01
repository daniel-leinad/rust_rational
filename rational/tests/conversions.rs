use rational::*;

#[test]
fn it_converts_from_integer() {
    let a: Rational = 1.into();
    let b = Rational::new(1, 1);
    assert_eq!(a, b);

    let a: Rational = 13.into();
    let b = Rational::new(13, 1);
    assert_eq!(a, b);

    let a: Rational = 0.into();
    assert_eq!(a, Rational::new(0, 1));
}

#[test]
fn it_converts_from_string_int() {
    let a: Rational = "1".parse().unwrap();
    let b = Rational::new(1, 1);
    assert_eq!(a, b);

    let a: Rational = "0".parse().unwrap();
    let b = Rational::new(0, 1);
    assert_eq!(a, b);

    let a: Rational = "01".parse().unwrap();
    let b = Rational::new(1, 1);
    assert_eq!(a, b);

    let a: Rational = "348763".parse().unwrap();
    let b = Rational::new(348763, 1);
    assert_eq!(a, b);
}

#[test]
fn it_converts_from_string_decimal() {
    let a: Rational = "1.5".parse().unwrap();
    let b = Rational::new(3, 2);
    assert_eq!(a, b);

    let a: Rational = "12.5".parse().unwrap();
    let b = Rational::new(125, 10);
    assert_eq!(a, b);

    let a: Rational = "0.5".parse().unwrap();
    let b = Rational::new(1, 2);
    assert_eq!(a, b);

    let a: Rational = "0.05".parse().unwrap();
    let b = Rational::new(1, 20);
    assert_eq!(a, b);

    let a: Rational = ".5".parse().unwrap();
    let b = Rational::new(1, 2);
    assert_eq!(a, b);

    let a: Rational = "5.".parse().unwrap();
    let b = Rational::new(5, 1);
    assert_eq!(a, b);

    let a: Rational = "0.".parse().unwrap();
    let b = Rational::new(0, 1);
    assert_eq!(a, b);
}

#[test]
fn it_converts_from_string_negative() {
    let a: Rational = "-1".parse().unwrap();
    let b = Rational::new(-1, 1);
    assert_eq!(a, b);

    let a: Rational = "-0".parse().unwrap();
    let b = Rational::new(-0, 1);
    assert_eq!(a, b);

    let a: Rational = "-01".parse().unwrap();
    let b = Rational::new(-1, 1);
    assert_eq!(a, b);

    let a: Rational = "-348763".parse().unwrap();
    let b = Rational::new(-348763, 1);
    assert_eq!(a, b);

    let a: Rational = "-1.5".parse().unwrap();
    let b = Rational::new(-3, 2);
    assert_eq!(a, b);

    let a: Rational = "-12.5".parse().unwrap();
    let b = Rational::new(-125, 10);
    assert_eq!(a, b);

    let a: Rational = "-0.5".parse().unwrap();
    let b = Rational::new(-1, 2);
    assert_eq!(a, b);

    let a: Rational = "-0.05".parse().unwrap();
    let b = Rational::new(-1, 20);
    assert_eq!(a, b);

    let a: Rational = "-.5".parse().unwrap();
    let b = Rational::new(-1, 2);
    assert_eq!(a, b);

    let a: Rational = "-5.".parse().unwrap();
    let b = Rational::new(-5, 1);
    assert_eq!(a, b);

    let a: Rational = "-0.".parse().unwrap();
    let b = Rational::new(0, 1);
    assert_eq!(a, b);
}

#[test]
fn it_doesnt_convert_from_string_negative_with_space() {
    let a: Result<Rational, _> = "- 1".parse();
    assert_eq!(a, Err("Error parsing string"));

    let a: Result<Rational, _> = "- 1.5".parse();
    assert_eq!(a, Err("Error parsing string"));
}

#[test]
fn cant_parse_empty_string() {
    let result: Result<Rational, _> = "".parse();
    assert_eq!(Err("Error parsing string"), result);
}

#[test]
fn cant_parse_incorrect_string_1() {
    let result: Result<Rational, _> = "sdjshdj".parse();
    assert_eq!(Err("Error parsing string"), result);
}

#[test]
fn cant_parse_incorrect_string_2() {
    let result: Result<Rational, _> = "1.2.3".parse();
    assert_eq!(Err("Error parsing string"), result);
}

#[test]
fn cant_parse_incorrect_string_3() {
    let result: Result<Rational, _> = "123-5.".parse();
    assert_eq!(Err("Error parsing string"), result);
}

#[test]
fn cant_parse_incorrect_string_4() {
    let result: Result<Rational, _> = "-".parse();
    assert_eq!(Err("Error parsing string"), result);
}

#[test]
fn doesnt_parse_two_periods() {
    let res: Result<Rational, _> = "1..5".parse();
    assert_eq!(res, Err("Error parsing string"));
}
