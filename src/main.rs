mod utils;
mod tests;
mod bonus_tests;

use std::env;
use tests::*;
use bonus_tests::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_bonus = args.len() > 1 && args[1] == "bonus";

    if is_bonus {
        run_complex_tests();
    } else {
        run_standard_tests();
    }
}

/// mandatory (f32)
fn run_standard_tests() {
    println!("--- Running Mandatory Tests (f32) ---");
    let tests: Vec<(&str, fn())> = vec![
        ("ex00: ", ex00), ("ex01: ", ex01), ("ex02: ", ex02),
        ("ex03: ", ex03), ("ex04: ", ex04), ("ex05: ", ex05),
        ("ex06: ", ex06), ("ex07: ", ex07), ("ex08: ", ex08),
        ("ex09: ", ex09), ("ex10: ", ex10), ("ex11: ", ex11),
        ("ex12: ", ex12), ("ex13: ", ex13), ("ex14: ", ex14),
    ];

    for (title, test) in tests {
        println!("Test {}", title);
        test();
    }
}

/// bonus(Complex)
fn run_complex_tests() {
    println!("--- Running Bonus Tests (Complex Number) ---");
    
    let bonus_tests: Vec<(&str, fn())> = vec![
        ("ex00 (Complex): ", bonus_ex00),
    ];

    for (title, test) in bonus_tests {
        println!("Test {}", title);
        test();
    }
}