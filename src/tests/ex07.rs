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
    let u = Matrix::from([
        [1., 0.],
        [0., 1.],
    ]);
    let v = Vector::from(vec![4., 2.]);
    println!("{}", u.mul_vec(&v));
    // [4.]
    // [2.]
    let u = Matrix::from([
    [2., 0.],
    [0., 2.],
    ]);
    let v = Vector::from(vec![4., 2.]);
    println!("{}", u.mul_vec(&v));
    // [8.]
    // [4.]
    let u = Matrix::from([
    [2., -2.],
    [-2., 2.],
    ]);
    let v = Vector::from(vec![4., 2.]);
    println!("{}", u.mul_vec(&v));
    // [4.]
    // [-4.]
    let u = Matrix::from([
    [1., 0.],
    [0., 1.],
    ]);
    let v = Matrix::from([
    [1., 0.],
    [0., 1.],]);
    println!("{}", u.mul_mat(&v));
    // [1., 0.]
    // [0., 1.]
    let u = Matrix::from([
    [1., 0.],
    [0., 1.],
    ]);
    let v = Matrix::from([
    [2., 1.],
    [4., 2.],
    ]);
    println!("{}", u.mul_mat(&v));
    // [2., 1.]
    // [4., 2.]
    let u = Matrix::from([
    [3., -5.],
    [6., 8.],
    ]);
    let v = Matrix::from([
    [2., 1.],
    [4., 2.],
    ]);
    println!("{}", u.mul_mat(&v));
    // [-14., -7.]
    // [44., 22.]
}
