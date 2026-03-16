/**
 * Exercise 07 - Linear map, Matrix multiplication
 * 
 * Allowed mathematical functions : fused multiply-add function
 * 
 * - Maximum time complexity : see below
 * - Maximum space complexity : see below
 * 
 * Let A ∈ Rm×n, B ∈ Rn×p and u ∈ Rn where (m, n, p) ∈ N3 (represented as variables of type u32)
    Au (which returns a vector in Rm) (max time complexity O(nm), max space com-
plexity O(nm))
    AB (which returns a matrix in Rm×p) (max time complexity O(nmp), max space
complexity O(nm + mp + np))
 */

use crate::utils::{Vector, Matrix};

pub fn ex07() {
    // let u = Matrix::from([
    //     [1., 0.],
    //     [0., 1.],
    // ]);
    // let v = Vector::from(vec![4., 2.]);
    // println!("{}", u.mul_vec(&v));
    // // [4.]
    // // [2.]
    // let u = Matrix::from([
    // [2., 0.],
    // [0., 2.],
    // ]);
    // let v = Vector::from(vec![4., 2.]);
    // println!("{}", u.mul_vec(&v));
    // // [8.]
    // // [4.]
    // let u = Matrix::from([
    // [2., -2.],
    // [-2., 2.],
    // ]);
    // let v = Vector::from(vec![4., 2.]);
    // println!("{}", u.mul_vec(&v));
    // // [4.]
    // // [-4.]
    // let u = Matrix::from([
    // [1., 0.],
    // [0., 1.],
    // ]);
    // let v = Matrix::from([
    // [1., 0.],
    // [0., 1.],]);
    // println!("{}", u.mul_mat(&v));
    // // [1., 0.]
    // // [0., 1.]
    // let u = Matrix::from([
    // [1., 0.],
    // [0., 1.],
    // ]);
    // let v = Matrix::from([
    // [2., 1.],
    // [4., 2.],
    // ]);
    // println!("{}", u.mul_mat(&v));
    // // [2., 1.]
    // // [4., 2.]
    // let u = Matrix::from([
    // [3., -5.],
    // [6., 8.],
    // ]);
    // let v = Matrix::from([
    // [2., 1.],
    // [4., 2.],
    // ]);
    // println!("{}", u.mul_mat(&v));
    // // [-14., -7.]
    // // [44., 22.]

    // 케이스 1: [[0, 0], [0, 0]]과 임의의 벡터 [4, 2] -> [0, 0]
    let u1 = Matrix::from([[0., 0.], [0., 0.]]);
    let v1 = Vector::from(vec![4., 2.]);
    println!("Case 1:\n{}", u1.mul_vec(&v1));

    // 케이스 2: 단위 행렬 [[1, 0], [0, 1]]과 임의의 벡터 [4, 2] -> [4, 2]
    let u2 = Matrix::from([[1., 0.], [0., 1.]]);
    let v2 = Vector::from(vec![4., 2.]);
    println!("Case 2:\n{}", u2.mul_vec(&v2));

    // 케이스 3: [[1, 1], [1, 1]]과 [4, 2] -> [6, 6]
    let u3 = Matrix::from([[1., 1.], [1., 1.]]);
    let v3 = Vector::from(vec![4., 2.]);
    println!("Case 3:\n{}", u3.mul_vec(&v3));

    // 케이스 4: [[2, 0], [0, 2]]와 [2, 1] -> [4, 2]
    let u4 = Matrix::from([[2., 0.], [0., 2.]]);
    let v4 = Vector::from(vec![2., 1.]);
    println!("Case 4:\n{}", u4.mul_vec(&v4));

    // 케이스 5: [[0.5, 0], [0, 0.5]]와 [4, 2] -> [2, 1]
    let u5 = Matrix::from([[0.5, 0.], [0., 0.5]]);
    let v5 = Vector::from(vec![4., 2.]);
    println!("Case 5:\n{}", u5.mul_vec(&v5));
}
