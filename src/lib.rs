use std::ops::{Add, Div, Mul, Neg, Sub};

type UnsignedInt = usize;
type SignedInt = isize;

#[derive(Debug, Copy, Clone)]
pub struct Rational {
    p: SignedInt,
    q: SignedInt,
}

impl Rational {
    pub fn new(p: SignedInt, q: SignedInt) -> Rational {
        if q == 0 { panic!("Denominator can't be zero!") };
        let mut res = Rational { p, q };
        if p != 0 { res.reduce() };
        res
    }

    fn reduce(&mut self) {
        let gcd = gcd(self.p.abs_diff(0), self.q.abs_diff(0)) as SignedInt;
        self.p = self.p / gcd;
        self.q = self.q / gcd;
    }
}

impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.p * other.q == self.q * other.p
    } 
}

impl From<SignedInt> for Rational {
    fn from(value: SignedInt) -> Self {
        Rational::new(value, 1)
    }
}

impl From<&str> for Rational {
    fn from(value: &str) -> Self {
        if value.len() == 0 { panic!("Error parsing string") };

        let mut numbers_str = String::with_capacity(value.len());
        let mut chars_iter = value.chars().peekable();
        let first_char = chars_iter.peek();
        let sign = if let Some('-') = first_char {
            chars_iter.next();
            -1
        } else {
            1
        };
        for char in &mut chars_iter {
            if char == '.' { break };
            if !char.is_digit(10) { panic!("Error parsing string") };
            numbers_str.push(char);
        };
        let mut decimal_power = 0;
        for char in chars_iter {
            if !char.is_digit(10) { panic!("Error parsing string") };
            numbers_str.push(char);
            decimal_power += 1;
        };
        let p: SignedInt = numbers_str.parse().expect("String must parse because it only contains digits, as ensured earlier");
        let q: SignedInt = (10 as SignedInt).pow(decimal_power);
        Rational::new(sign * p, q)
    }
}

impl Mul for Rational {
    type Output = Rational;

    fn mul(self, rhs: Self) -> Self::Output {
        Rational::new(self.p * rhs.p, self.q * rhs.q)
    }
}

impl Div for Rational {
    type Output = Rational;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs == 0.into() { panic!("Can't divide by zero") };
        Rational::new(self.p * rhs.q, self.q * rhs.p)
    }
}

impl Add for Rational {
    type Output = Rational;

    fn add(self, rhs: Self) -> Self::Output {
        Rational::new((self.p * rhs.q) + (rhs.p * self.q), self.q * rhs.q)
    }
}

impl Neg for Rational {
    type Output = Rational;

    fn neg(self) -> Self::Output {
        let Rational {p, q} = self;
        Rational {
            p: -p,
            q: q,
        }
    }
}

impl Sub for Rational {
    type Output = Rational;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}



// struct RawDecimalParts<'a> {
//     int: &'a str,
//     decimal: &'a str,
// }

// impl<'a> RawDecimalParts<'a> {
//     fn from_str(value: &'a str) -> Self {
//         let int_end = None;
//         let mut chars_iter = value.char_indices();
//         for (idx, char) in &mut chars_iter {
//             if char == '.' {
//                 int_end = Some(idx);
//                 break;
//             };
            
//             if !char.is_digit(10) { 
//                 panic!("Error parsing string"); 
//             };
//         };
//         let int_end = int_end.
//         for (_, char) in chars_iter {
//             if !char.is_digit(10) { panic!("Error parsing string") };
//         };

//         let (int_str, remainder) = value.split_at(int_end);
//         let decimal_str = remainder.strip_prefix(".").expect("Must start with \".\" because it would've panicked earlier");
//         RawDecimalParts { int: int_str, decimal: decimal_str}
//     }
// }

fn gcd(mut a: UnsignedInt, mut b: UnsignedInt) -> UnsignedInt {
    while a != b {
        if a > b {
            a = a - b;
        } else {
            b = b - a;
        }
    }

    return a
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn zero_is_zero() {
        let a =  Rational::new(0, 1);
        let b =  Rational::new(0, -1343535);
        assert_eq!(a, b);
    }

    #[should_panic(expected = "Denominator can't be zero!")]
    #[test]
    fn cant_make_denominator_zero() {
        let _result = Rational::new(1, 0);
    }

    #[test]
    fn it_reduces() {
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
    fn it_equates() {
        let a = Rational::new(15, 5);
        let b = Rational::new(15, 5);
        assert_eq!(a, b);

        let a = Rational::new(15, 5);
        let b = Rational::new(3, 1);
        assert_eq!(a, b);

        let a = Rational::new(15, -5);
        let b = Rational::new(-15, 5);
        assert_eq!(a, b);

        let a = Rational::new(15, 5);
        let b = Rational::new(-3, -1);
        assert_eq!(a, b);
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
        let a: Rational = "1".into();
        let b = Rational::new(1, 1);
        assert_eq!(a, b);

        let a: Rational = "0".into();
        let b = Rational::new(0, 1);
        assert_eq!(a, b);

        let a: Rational = "01".into();
        let b = Rational::new(1, 1);
        assert_eq!(a, b);

        let a: Rational = "348763".into();
        let b = Rational::new(348763, 1);
        assert_eq!(a, b);
    }

    #[test]
    fn it_converts_from_string_decimal() {
        let a: Rational = "1.5".into();
        let b = Rational::new(3, 2);
        assert_eq!(a, b);

        let a: Rational = "12.5".into();
        let b = Rational::new(125, 10);
        assert_eq!(a, b);

        let a: Rational = "0.5".into();
        let b = Rational::new(1, 2);
        assert_eq!(a, b);

        let a: Rational = "0.05".into();
        let b = Rational::new(1, 20);
        assert_eq!(a, b);

        let a: Rational = ".5".into();
        let b = Rational::new(1, 2);
        assert_eq!(a, b);

        let a: Rational = "5.".into();
        let b = Rational::new(5, 1);
        assert_eq!(a, b);

        let a: Rational = "0.".into();
        let b = Rational::new(0, 1);
        assert_eq!(a, b);
    }

    #[test]
    fn it_converts_from_string_negative() {
        let a: Rational = "-1".into();
        let b = Rational::new(-1, 1);
        assert_eq!(a, b);

        let a: Rational = "-0".into();
        let b = Rational::new(-0, 1);
        assert_eq!(a, b);

        let a: Rational = "-01".into();
        let b = Rational::new(-1, 1);
        assert_eq!(a, b);

        let a: Rational = "-348763".into();
        let b = Rational::new(-348763, 1);
        assert_eq!(a, b);

        let a: Rational = "-1.5".into();
        let b = Rational::new(-3, 2);
        assert_eq!(a, b);

        let a: Rational = "-12.5".into();
        let b = Rational::new(-125, 10);
        assert_eq!(a, b);

        let a: Rational = "-0.5".into();
        let b = Rational::new(-1, 2);
        assert_eq!(a, b);

        let a: Rational = "-0.05".into();
        let b = Rational::new(-1, 20);
        assert_eq!(a, b);

        let a: Rational = "-.5".into();
        let b = Rational::new(-1, 2);
        assert_eq!(a, b);

        let a: Rational = "-5.".into();
        let b = Rational::new(-5, 1);
        assert_eq!(a, b);

        let a: Rational = "-0.".into();
        let b = Rational::new(0, 1);
        assert_eq!(a, b);
    }

    #[should_panic(expected = "Error parsing string")]
    #[test]
    fn cant_parse_empty_string() {
        let _result: Rational = "".into();
    }

    #[should_panic(expected = "Error parsing string")]
    #[test]
    fn cant_parse_incorrect_string_1() {
        let _res: Rational = "sdjshdj".into();
    }

    #[should_panic(expected = "Error parsing string")]
    #[test]
    fn cant_parse_incorrect_string_2() {
        let _res: Rational = "1.2.3".into();
    }

    #[should_panic(expected = "Error parsing string")]
    #[test]
    fn cant_parse_incorrect_string_3() {
        let _res: Rational = "123-5.".into();
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

    #[should_panic(expected="Can't divide by zero")]
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
        check_addition("2.3".into(), "10.185".into(), "12.485".into());
    }

    #[test]
    fn it_negates() {
        let a: Rational = 5.into();
        assert_eq!(-a, (-5).into());
    }

    #[test]
    fn it_substracts() {
        let a: Rational = 5.into();
        let b = 0.into();
        check_substraction(a, b, 5.into());

        let a: Rational = 5.into();
        let b = 3.into();
        check_substraction(a, b, 2.into());

        let a: Rational = 5.into();
        let b = (-3).into();
        check_substraction(a, b, 8.into());

        let a: Rational = "0.3".into();
        let b = "0.1".into();
        check_substraction(a, b, "0.2".into());
    }

    fn check_addition(a: Rational, b: Rational, res: Rational) {

        assert_eq!(a + b, res);
        assert_eq!(b + a, res);

    }

    fn check_multiplication(a: Rational, b: Rational, res: Rational) {

        assert_eq!(a * b, res);
        assert_eq!(b * a, res);

    }

    fn check_substraction(a: Rational, b: Rational, res: Rational) {

        assert_eq!(a - b, res);
        assert_eq!(b - a, -res);

    }
}
