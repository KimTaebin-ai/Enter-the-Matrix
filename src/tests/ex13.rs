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
    let u = Matrix::from([
    [1., 0., 0.],
    [0., 1., 0.],
    [0., 0., 1.],
    ]);
    println!("{}", u.rank());
    // 3
    let u = Matrix::from([
    [ 1., 2., 0., 0.],
    [ 2., 4., 0., 0.],
    [-1., 2., 1., 1.],
    ]);
    println!("{}", u.rank());
    // 2
    let u = Matrix::from([
    [ 8., 5., -2.],
    [ 4., 7., 20.],
    [ 7., 6., 1.],
    [21., 18., 7.],
    ]);
    println!("{}", u.rank());
    // 3
}