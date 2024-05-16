mod generated;

use std::time::Instant;

fn main() {
    measure_func(generated::using_parse_int, "using_parse_int");
    measure_func(generated::using_macro_int, "using_macro_int");

    measure_func(generated::using_parse_float, "using_parse_float");
    measure_func(generated::using_macro_float, "using_macro_float");
    
    measure_func(generated::using_parse_float_2, "using_parse_float_2");
    measure_func(generated::using_macro_float_2, "using_macro_float_2");

    measure_func(generated::using_parse_repeating, "using_parse_repeating");
    measure_func(generated::using_macro_repeating, "using_macro_repeating");

    measure_func(generated::addition, "addition");
    measure_func(generated::subtraction, "subtraction");
    measure_func(generated::multiplication, "multiplication");
    measure_func(generated::division, "division");
}

fn measure_func(mut func: impl FnMut() -> (), name: &str) {
    let start_time = Instant::now();
    for _ in 0..10000 {
        func();
    }
    let elapsed = start_time.elapsed();
    println!("benchmark {name}: {elapsed:?}")
}