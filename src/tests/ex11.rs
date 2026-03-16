/**
 * Exercise 11 - Determinant
 * 
 * Allowed mathematical functions : fused multiply-add function
 * 
 * - Maximum time complexity : O(n^3)
 * - Maximum space complexity : O(n^2)
 */

use crate::utils::{Matrix};

pub fn ex11() {
    // let u = Matrix::from([
    // [ 1., -1.],
    // [-1., 1.],
    // ]);
    // println!("{}", u.determinant());
    // // 0.0
    // let u = Matrix::from([
    // [2., 0., 0.],
    // [0., 2., 0.],
    // [0., 0., 2.],
    // ]);
    // println!("{}", u.determinant());
    // // 8.0
    // let u = Matrix::from([
    // [8., 5., -2.],
    // [4., 7., 20.],
    // [7., 6., 1.],
    // ]);
    // println!("{}", u.determinant());
    // // -174.0
    // let u = Matrix::from([
    // [ 8., 5., -2., 4.],
    // [ 4., 2.5, 20., 4.],
    // [ 8., 5., 1., 4.],
    // [28., -4., 17., 1.],
    // ]);
    // println!("{}", u.determinant());
    // // 1032

    // 케이스 1: 2x2 영행렬 -> (0*0) - (0*0) = 0
    let u1 = Matrix::from([[0., 0.], [0., 0.]]);
    println!("Case 1: {}", u1.determinant());

    // 케이스 2: 2x2 단위 행렬 -> (1*1) - (0*0) = 1
    let u2 = Matrix::from([[1., 0.], [0., 1.]]);
    println!("Case 2: {}", u2.determinant());

    // 케이스 3: [[2, 0], [0, 2]] -> (2*2) - (0*0) = 4
    let u3 = Matrix::from([[2., 0.], [0., 2.]]);
    println!("Case 3: {}", u3.determinant());

    // 케이스 4: [[1, 1], [1, 1]] -> (1*1) - (1*1) = 0 (특이 행렬)
    let u4 = Matrix::from([[1., 1.], [1., 1.]]);
    println!("Case 4: {}", u4.determinant());

    // 케이스 5: [[0, 1], [1, 0]] -> (0*0) - (1*1) = -1
    let u5 = Matrix::from([[0., 1.], [1., 0.]]);
    println!("Case 5: {}", u5.determinant());

    // 케이스 6: [[1, 2], [3, 4]] -> (1*4) - (2*3) = -2
    let u6 = Matrix::from([[1., 2.], [3., 4.]]);
    println!("Case 6: {}", u6.determinant());

    // 케이스 7: [[-7, 5], [4, 6]] -> (-7*6) - (5*4) = -42 - 20 = -62
    let u7 = Matrix::from([[-7., 5.], [4., 6.]]);
    println!("Case 7: {}", u7.determinant());

    // 케이스 8: 3x3 단위 행렬 -> 1
    let u8 = Matrix::from(vec![
        vec![1., 0., 0.],
        vec![0., 1., 0.],
        vec![0., 0., 1.],
    ]);
    println!("Case 8: {}", u8.determinant());
}