/**
 * Exercise 09 - Transpose
 * 
 * Allowed mathematical functions : None
 * 
 * - Maximum time complexity : O(nm)
 * - Maximum space complexity : O(nm)
 */

use crate::utils::{Matrix};

pub fn ex09() {
    let u = Matrix::from([
        [1., 2.], 
        [3., 4.]
    ]);
    println!("{}", u.transpose());
    // [1., 3.]
    // [2., 4.]
    let u = Matrix::from([
        [1., 2.], 
        [3., 4.], 
        [5., 6.]
    ]);
    println!("{}", u.transpose());
    // [1., 3., 5.]
    // [2., 4., 6.]
}