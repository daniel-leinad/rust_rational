mod generated;

use std::time::Instant;

fn main() {
    measure_func(generated::using_parse_int, "using_parse_int");
    measure_func(generated::using_parse_float, "using_parse_float");
    measure_func(generated::using_macro_int, "using_macro_int");
    measure_func(generated::using_macro_float, "using_macro_float");
}

fn measure_func(mut func: impl FnMut() -> (), name: &str) {
    let start_time = Instant::now();
    for _ in 0..1000 {
        func();
    }
    let elapsed = start_time.elapsed();
    println!("benchmark {name}: {elapsed:?}")
}