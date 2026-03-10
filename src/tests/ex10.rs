/**
 * Exercise 10 - row-echelon form
 * 
 * Allowed mathematical functions : None
 * 
 * - Maximum time complexity : O(n^3)
 * - Maximum space complexity : O(n^2)
 */

use crate::utils::{Matrix};

pub fn ex10() {
    let u = Matrix::from([
    [1., 0., 0.],
    [0., 1., 0.],
    [0., 0., 1.],
    ]);
    println!("{}", u.row_echelon());
    // [1.0, 0.0, 0.0]
    // [0.0, 1.0, 0.0]
    // [0.0, 0.0, 1.0]
    let u = Matrix::from([
    [1., 2.],
    [3., 4.],
    ]);
    println!("{}", u.row_echelon());
    // [1.0, 0.0]
    // [0.0, 1.0]
    let u = Matrix::from([
    [1., 2.],
    [2., 4.],
    ]);
    println!("{}", u.row_echelon());
    // [1.0, 2.0]
    // [0.0, 0.0]
    let u = Matrix::from([
    [8., 5., -2., 4., 28.],
    [4., 2.5, 20., 4., -4.],
    [8., 5., 1., 4., 17.],
    ]);
    println!("{}", u.row_echelon());
    // [1.0, 0.625, 0.0, 0.0, -12.1666667]
    // [0.0, 0.0, 1.0, 0.0, -3.6666667]
    // [0.0, 0.0, 0.0, 1.0, 29.5 ]
}