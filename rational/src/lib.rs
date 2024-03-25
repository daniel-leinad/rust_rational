// TODO handle overflows
#[cfg(test)]
mod tests;

use std::cmp::min;
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
        if q == 0 { panic!("Denominator can't be zero!") };
        let mut res = Rational { p, q };
        if p != 0 { res.reduce() };
        res
    }

    /// Creates a new Rational without reducing the fraction, 
    /// therefore should only be used when you are 100% certain numerator and denominator are reduced 
    /// Can be used for optimisations
    /// Panics when q == 0
    pub fn new_reduced(p: SignedInt, q: SignedInt) -> Rational {
        if q == 0 { panic!("Denominator can't be zero!") };
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
        let mut numbers_str = String::with_capacity(value.len());
        let mut chars_iter = value.chars().peekable();
        let first_char = chars_iter.peek();
        
        let sign = match first_char {
            Some('-') => {
                chars_iter.next();
                let second_char = chars_iter.peek();
                if let Some(' ') = second_char { chars_iter.next(); };
                -1
            },
            _ => {1}
        };
        
        for char in &mut chars_iter {
            if char == '.' { break };
            if !char.is_digit(10) { return Err("Error parsing string") };
            numbers_str.push(char);
        };
        let mut decimal_power = 0;
        for char in chars_iter {
            if !char.is_digit(10) { return Err("Error parsing string") };
            numbers_str.push(char);
            decimal_power += 1;
        };

        if numbers_str.len() == 0 { return Err("Error parsing string") };

        // TODO: there are still possible parsing errors like PosOverFlow that should be handled instead of panicking
        let p: SignedInt = numbers_str.parse().expect("String must parse because it is non-empty and only contains digits, as ensured earlier");
        let q: SignedInt = (10 as SignedInt).pow(decimal_power);
        Ok(Rational::new(sign * p, q))
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

fn gcd(mut a: UnsignedInt, mut b: UnsignedInt) -> UnsignedInt {

    // Simple case optimization
    if min(a, b) == 1 { return 1 };

    let mut d = 1;

    // Bitshift optimization technique
    while (a % 2 == 0) && (b % 2 == 0) {
        a = a / 2;
        b = b / 2;
        d = d * 2;
    };
    
    while a % 2 == 0 {
        a = a / 2;
    };

    while b % 2 == 0 {
        b = b / 2;
    };

    // Euclid's algorithm
    while (a != 0) && (b != 0) {
        if a > b {
            a = a % b;
        } else {
            b = b % a;
        }
    }

    return (a + b) * d
}
