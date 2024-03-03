use proc_macro::TokenStream;
use rational::Rational;
use quote::quote;
use std::stringify;

#[proc_macro]
pub fn rational(input: TokenStream) -> TokenStream {
    let str = input.to_string();
    let rational: Rational = str.parse().expect(&format!("Incorrect rational literal: {str}"));
    let (p, q) = (rational.numerator(), rational.denominator());

    let expanded = quote!{
        Rational::new(#p, #q)
    };

    TokenStream::from(expanded)
}

// use proc_macro;

// #[proc_macro_derive(MyDerive)]
// pub fn rational(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let input = proc_macro2::TokenStream::from(input);

//     let str = input.to_string();

//     println!("{str}");

//     let output: proc_macro2::TokenStream = proc_macro2::TokenStream::new();

//     proc_macro::TokenStream::from(output)
// }