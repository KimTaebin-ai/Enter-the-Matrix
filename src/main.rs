// K 라는 타입을 담을 건데, 이건 나중에 정하겠음
// K 는 abstract algebra 로 봤을 때 scalar 의 체로 볼 수 있음.

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
        ("ex06: ", ex06),
        ("ex07: ", ex07),
        ("ex08: ", ex08),
        ("ex09: ", ex09),
        ("ex10: ", ex10),
        ("ex11: ", ex11),
        ("ex12: ", ex12),
        ("ex13: ", ex13),
        ("ex14: ", ex14),
    ];

    for (title, test) in tests {
        println!("Test {}", title);
        test();
    }
}
