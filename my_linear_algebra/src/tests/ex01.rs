/**
 * Exercise 01 - Linear combination
 * 
 * Allowed mathematical functions : fused multiply-add function
 * 
 * - Maximum time complexity : O(n)
 * - Maximum space complexity : O(n)
 */

use crate::utils::{Vector};
use crate::utils::vector::linear_combination;

pub fn ex01() {
    let e1 = Vector::from(vec![1., 0., 0.]);
    let e2 = Vector::from(vec![0., 1., 0.]);
    let e3 = Vector::from(vec![0., 0., 1.]);
    
    let v1 = Vector::from(vec![1., 2., 3.]);
    let v2 = Vector::from(vec![0., 10., -100.]);
    
    println!("{}", linear_combination(
        &vec![e1, e2, e3], 
        &vec![10., -2., 0.5]));
    // // [10.0]
    // // [-2.0]
    // // [0.5]
    println!("{}", linear_combination(
        &vec![v1, v2], 
        &vec![10., -2.]));
    // [10.0]
    // [0.0]
    // [230.0]
}