// TODO handle overflows
#[cfg(test)]
mod tests;

use std::borrow::Borrow;
use std::cmp::min;
use std::iter::Peekable;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::FromStr;

type UnsignedInt = usize;
type SignedInt = isize;

#[derive(Debug, Copy, Clone)]
pub struct Rational {
    p: SignedInt,
    q: SignedInt,
}

impl Rational {
    /// Builds a new rational and reduces the undelying fraction
    /// Panics when q == 0
    pub fn new(p: SignedInt, q: SignedInt) -> Rational {
        if q == 0 {
            panic!("Denominator can't be zero!")
        };
        let mut res = Rational { p, q };
        if p != 0 {
            res.reduce()
        };
        res
    }

    /// Creates a new Rational without reducing the fraction,
    /// therefore should only be used when you are 100% certain numerator and denominator are reduced
    /// Can be used for optimisations
    /// Panics when q == 0
    pub fn new_reduced(p: SignedInt, q: SignedInt) -> Rational {
        if q == 0 {
            panic!("Denominator can't be zero!")
        };
        Rational { p, q }
    }

    fn reduce(&mut self) {
        let gcd = gcd(self.p.abs_diff(0), self.q.abs_diff(0)) as SignedInt;
        self.p = self.p / gcd;
        self.q = self.q / gcd;
    }

    pub fn numerator(&self) -> SignedInt {
        self.p
    }

    pub fn denominator(&self) -> SignedInt {
        self.q
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

impl FromStr for Rational {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        type DigitsSequence = str;
        type ParsingError = &'static str;

        fn parse_sign(chars_iter: &mut Peekable<impl Iterator<Item = char>>) -> SignedInt {
            let first_char = chars_iter.peek();

            match first_char {
                Some('-') => {
                    chars_iter.next();
                    -1
                }
                _ => 1,
            }
        }

        fn parse_integral_part(
            chars_iter: &mut Peekable<impl Iterator<Item = char>>,
            capacity: usize,
        ) -> Result<impl Borrow<DigitsSequence>, ParsingError> {
            
            let mut res = String::with_capacity(capacity);

            while let Some(char) = chars_iter.peek() {
                if char.is_digit(10) {
                    res.push(*char);
                    chars_iter.next();
                } else {
                    break;
                }
            };

            Ok(res)
        }

        fn parse_fractional_part(
            chars_iter: &mut Peekable<impl Iterator<Item = char>>,
            capacity: usize,
        ) -> Result<impl Borrow<DigitsSequence>, ParsingError> {

            match chars_iter.peek() {
                Some('.') => chars_iter.next(),
                None => return Ok(String::new()), 
                _ => return Err("Error parsing string")
            };

            let mut res = String::with_capacity(capacity);

            while let Some(char) = chars_iter.peek() {
                if char.is_digit(10) {
                    res.push(*char);
                    chars_iter.next();
                } else {
                    break;
                }
            };

            Ok(res)
        }

        fn parse_repeating_part(
            chars_iter: &mut Peekable<impl Iterator<Item = char>>,
            capacity: usize,
        ) -> Result<impl Borrow<DigitsSequence>, ParsingError> {
            match chars_iter.peek() {
                Some('(') => chars_iter.next(),
                None => return Ok(String::new()),
                _ => return Err("Error parsing string")
            };

            let mut res = String::with_capacity(capacity);

            while let Some(char) = chars_iter.peek() {
                if char.is_digit(10) {
                    res.push(*char);
                    chars_iter.next();
                } else {
                    break;
                }
            };

            if res.len() == 0 { return Err("Error parsing string") };

            match chars_iter.next() {
                Some(')') => Ok(res),
                _ => Err("Error parsing string")
            }
        }

        fn parse_digits_sequence(digits_sequence: &DigitsSequence) -> UnsignedInt {
            // TODO: there are still possible parsing errors like PosOverFlow that should be handled instead of panicking
            digits_sequence
                .parse()
                .expect("String must parse because it is non-empty and only contains digits, as ensured earlier")
        }

        fn get_p_q(
            integral_part: &DigitsSequence,
            fractional_part: &DigitsSequence,
        ) -> (UnsignedInt, UnsignedInt) {
            let both_parts = [integral_part, fractional_part].concat();

            if both_parts.len() == 0 {
                return (0, 1);
            }

            let p = parse_digits_sequence(&both_parts);

            let q = (10 as UnsignedInt).pow(fractional_part.len() as u32);
            (p, q)
        }

        fn get_repeating_p_q(repeating_part: &DigitsSequence) -> (UnsignedInt, UnsignedInt) {
            if repeating_part.len() == 0 {
                return (0, 1);
            };

            let p = parse_digits_sequence(repeating_part);

            let mut q = 0;

            for _ in 0..repeating_part.len() {
                // TODO possible overflow
                q *= 10;
                q += 9;
            }

            (p, q)
        }

        let capacity = value.len();
        let mut chars_iter = value.chars().peekable();

        let sign = parse_sign(&mut chars_iter);
        let integral_part = parse_integral_part(&mut chars_iter, capacity)?;
        let fractional_part = parse_fractional_part(&mut chars_iter, capacity)?;
        let repeating_part = parse_repeating_part(&mut chars_iter, capacity)?;

        // make sure iterator is exhausted
        if let Some(_) = chars_iter.next() {
            return Err("Error parsing string")
        }

        if integral_part.borrow().len() + fractional_part.borrow().len() + repeating_part.borrow().len() == 0 {
            return Err("Error parsing string");
        };

        let (p, q) = get_p_q(integral_part.borrow(), fractional_part.borrow());
        let (repeating_p, repeating_q) = get_repeating_p_q(repeating_part.borrow());

        Ok(Rational::new(sign * p as SignedInt, q as SignedInt) + 
           Rational::new(sign * repeating_p as SignedInt,(repeating_q * q) as SignedInt))
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
        if rhs == 0.into() {
            panic!("Can't divide by zero")
        };
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
        let Rational { p, q } = self;
        Rational { p: -p, q: q }
    }
}

impl Sub for Rational {
    type Output = Rational;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

fn gcd(mut a: UnsignedInt, mut b: UnsignedInt) -> UnsignedInt {
    // Simple case optimization
    if min(a, b) == 1 { return 1 };

    let mut d = 1;

    // Bitshift optimization technique
    while (a % 2 == 0) && (b % 2 == 0) {
        a = a / 2;
        b = b / 2;
        d = d * 2;
    }

    while a % 2 == 0 {
        a = a / 2;
    }

    while b % 2 == 0 {
        b = b / 2;
    }

    // Euclid's algorithm
    while (a != 0) && (b != 0) {
        if a > b {
            a = a % b;
        } else {
            b = b % a;
        }
    }

    return (a + b) * d;
}
