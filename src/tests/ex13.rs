/**
 * Exercise 13 - Rank
 * 
 * Allowed mathematical functions : None
 * 
 * - Maximum time complexity : O(n^3)
 * - Maximum space complexity : N/A
 */

use crate::utils::{Matrix};

pub fn ex13() {
    // let u = Matrix::from([
    // [1., 0., 0.],
    // [0., 1., 0.],
    // [0., 0., 1.],
    // ]);
    // println!("{}", u.rank());
    // // 3
    // let u = Matrix::from([
    // [ 1., 2., 0., 0.],
    // [ 2., 4., 0., 0.],
    // [-1., 2., 1., 1.],
    // ]);
    // println!("{}", u.rank());
    // // 2
    // let u = Matrix::from([
    // [ 8., 5., -2.],
    // [ 4., 7., 20.],
    // [ 7., 6., 1.],
    // [21., 18., 7.],
    // ]);
    // println!("{}", u.rank());
    // // 3

    // 케이스 1: [[0, 0], [0, 0]] -> 모든 행이 0이므로 Rank 0
    let u1 = Matrix::from([[0., 0.], [0., 0.]]);
    println!("Case 1: {}", u1.rank());

    // 케이스 2: 2x2 단위 행렬 -> 두 행이 모두 독립적이므로 Rank 2
    let u2 = Matrix::from([[1., 0.], [0., 1.]]);
    println!("Case 2: {}", u2.rank());

    // 케이스 3: [[2, 0], [0, 2]] -> Rank 2
    let u3 = Matrix::from([[2., 0.], [0., 2.]]);
    println!("Case 3: {}", u3.rank());

    // 케이스 4: [[1, 1], [1, 1]] -> 두 행이 동일(종속적)하므로 Rank 1
    let u4 = Matrix::from([[1., 1.], [1., 1.]]);
    println!("Case 4: {}", u4.rank());

    // 케이스 5: [[0, 1], [1, 0]] -> Rank 2
    let u5 = Matrix::from([[0., 1.], [1., 0.]]);
    println!("Case 5: {}", u5.rank());

    // 케이스 6: [[1, 2], [3, 4]] -> Rank 2
    let u6 = Matrix::from([[1., 2.], [3., 4.]]);
    println!("Case 6: {}", u6.rank());

    // 케이스 7: [[-7, 5], [4, 6]] -> Rank 2
    let u7 = Matrix::from([[-7., 5.], [4., 6.]]);
    println!("Case 7: {}", u7.rank());

    // 케이스 8: 3x3 단위 행렬 -> Rank 3
    let u8 = Matrix::from(vec![
        vec![1., 0., 0.],
        vec![0., 1., 0.],
        vec![0., 0., 1.],
    ]);
    println!("Case 8: {}", u8.rank());
}