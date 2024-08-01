//! This crate provides proc-macros for the [rational](../rational/index.html) crate.

use proc_macro::TokenStream;
use quote::quote;
use rational::Rational;

/// Build a rational number with checks and internal fraction reduction at compile time.
///
/// The rational number can currently only be built from a decimal representation
///
/// ```
/// # use rational_proc_macro::rational;
/// # use rational::Rational;
/// let a = rational!(5);
/// assert_eq!(a, Rational::new(5, 1));
///
/// let b = rational!(1.5);
/// assert_eq!(b, Rational::new(3, 2));
///
/// let c = rational!{0.(6)};
/// assert_eq!(c, Rational::new(2, 3));
///
/// // let d = rational!(1, 3); // will not compile
/// // let e = rational!(1 / 3); // will not compile
/// ```
#[proc_macro]
pub fn rational(input: TokenStream) -> TokenStream {
    // "-1.3(5)" is tokenized as "- 1.3 (5)", so we need to get rid of the whitespaces
    let str = input.to_string().replace("- ", "-").replace(" (", "(");

    let rational: Rational = str
        .parse()
        .unwrap_or_else(|_| panic!("Incorrect rational literal: {str}"));
    let (p, q) = (rational.numerator(), rational.denominator());

    let expanded = quote! {
        Rational::new_unchecked(#p, #q)
    };

    TokenStream::from(expanded)
}
