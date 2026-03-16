/**
 * Exercise 12 - Inverse
 * 
 * Allowed mathematical functions : fused multiply-add function
 * 
 * - Maximum time complexity : O(n^3)
 * - Maximum space complexity : O(n^2)
 */

use crate::utils::{Matrix};

pub fn ex12() {
    // let u = Matrix::from([
    // [1., 0., 0.],
    // [0., 1., 0.],
    // [0., 0., 1.],
    // ]);
    // println!("{}", u.inverse());
    // // [1.0, 0.0, 0.0]
    // // [0.0, 1.0, 0.0]
    // // [0.0, 0.0, 1.0]
    // let u = Matrix::from([
    // [2., 0., 0.],
    // [0., 2., 0.],
    // [0., 0., 2.],
    // ]);
    // println!("{}", u.inverse());
    // // [0.5, 0.0, 0.0]
    // // [0.0, 0.5, 0.0]
    // // [0.0, 0.0, 0.5]
    // let u = Matrix::from([
    // [8., 5., -2.],
    // [4., 7., 20.],
    // [7., 6., 1.],
    // ]);
    // println!("{}", u.inverse());
    // // [0.649425287, 0.097701149, -0.655172414]
    // // [-0.781609195, -0.126436782, 0.965517241]
    // // [0.143678161, 0.074712644, -0.206896552]

    // 케이스 1: 2x2 단위 행렬의 역행렬 -> 자기 자신
    let u1 = Matrix::from([[1., 0.], [0., 1.]]);
    println!("Case 1:\n{}", u1.inverse());

    // 케이스 2: [[2, 0], [0, 2]] -> [[0.5, 0], [0, 0.5]]
    let u2 = Matrix::from([[2., 0.], [0., 2.]]);
    println!("Case 2:\n{}", u2.inverse());

    // 케이스 3: [[0.5, 0], [0, 0.5]] -> [[2, 0], [0, 2]]
    let u3 = Matrix::from([[0.5, 0.], [0., 0.5]]);
    println!("Case 3:\n{}", u3.inverse());

    // 케이스 4: [[0, 1], [1, 0]] -> [[0, 1], [1, 0]]
    let u4 = Matrix::from([[0., 1.], [1., 0.]]);
    println!("Case 4:\n{}", u4.inverse());

    // 케이스 5: [[1, 2], [3, 4]] -> [[-2, 1], [1.5, -0.5]]
    let u5 = Matrix::from([[1., 2.], [3., 4.]]);
    println!("Case 5:\n{}", u5.inverse());

    // 케이스 6: 3x3 단위 행렬 -> 자기 자신
    let u6 = Matrix::from(vec![
        vec![1., 0., 0.],
        vec![0., 1., 0.],
        vec![0., 0., 1.],
    ]);
    println!("Case 6:\n{}", u6.inverse());
}