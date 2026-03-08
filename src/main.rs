// K 라는 타입을 담을 건데, 이건 나중에 정하겠음
// K 는 abstract algebra 로 봤을 때 scalar 의 체로 볼 수 있음.

/**
 * We recommend you to implement some utility functions, such as:
• A function to return the size/shape of a vector/matrix.
• A function to tell if a matrix is square.
• A function to print a vector/matrix on the standard output.
• A function to reshape a vector into a matrix, and vice-versa.
 */

/**
 * Display notation 관련
 * ‘\u{23A1}’, ‘\u{23A2}’, ‘\u{23A3}’, ‘\u{23A4}’, ‘\u{23A6}’
 */

mod utils;
mod tests;

use tests::*;

fn main() {
    let tests: Vec<(&str, fn())> = vec![
        ("ex00: ", ex00),
        ("ex01: ", ex01),
        ("ex02: ", ex02),
        ("ex03: ", ex03),
        ("ex04: ", ex04),
        ("ex05: ", ex05),
    ];

    for (title, test) in tests {
        println!("Test {}", title);
        test();
    }
}
