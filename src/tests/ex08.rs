/**
 * Exercise 08 - Trace
 * 
 * Allowed mathematical functions : None
 * 
 * - Maximum time complexity : O(n)
 * - Maximum space complexity : N/A
 */

use crate::utils::{Matrix};

pub fn ex08() {
    // let u = Matrix::from([
    // [1., 0.],
    // [0., 1.],
    // ]);
    // println!("{}", u.trace());
    // // 2.0
    // let u = Matrix::from([
    // [2., -5., 0.],
    // [4., 3., 7.],
    // [-2., 3., 4.],
    // ]);
    // println!("{}", u.trace());
    // // 9.0
    // let u = Matrix::from([
    // [-2., -8., 4.],
    // [1., -23., 4.],
    // [0., 6., 4.],
    // ]);
    // println!("{}\n", u.trace());
    // // -21.0

    // 케이스 1: [[0, 0], [0, 0]] -> 0.0 + 0.0 = 0.0
    let u1 = Matrix::from([
        [0., 0.],
        [0., 0.]
    ]);
    println!("Case 1: {}", u1.trace());

    // 케이스 2: [[1, 0], [0, 1]] -> 1.0 + 1.0 = 2.0
    let u2 = Matrix::from([
        [1., 0.],
        [0., 1.]
    ]);
    println!("Case 2: {}", u2.trace());

    // 케이스 3: [[1, 2], [3, 4]] -> 1.0 + 4.0 = 5.0
    let u3 = Matrix::from([
        [1., 2.],
        [3., 4.]
    ]);
    println!("Case 3: {}", u3.trace());

    // 케이스 4: [[8, -7], [4, 2]] -> 8.0 + 2.0 = 10.0
    let u4 = Matrix::from([
        [8., -7.],
        [4., 2.]
    ]);
    println!("Case 4: {}", u4.trace());

    // 케이스 5: 3x3 단위 행렬 -> 1.0 + 1.0 + 1.0 = 3.0
    // Matrix::from이 가변 크기를 지원하도록 구현되어 있어야 합니다.
    let u5 = Matrix::from(vec![
        vec![1., 0., 0.],
        vec![0., 1., 0.],
        vec![0., 0., 1.],
    ]);
    println!("Case 5: {}", u5.trace());
}