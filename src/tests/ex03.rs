/**
 * Exercise 03 - Dot product
 * 
 * Allowed mathematical functions : fused multiply-add function
 * 
 * - Maximum time complexity : O(n)
 * - Maximum space complexity : O(n)
 */

use crate::utils::{Vector};

pub fn ex03() {
    let mut u = Vector::from(vec![0., 0.]);
    let v = Vector::from(vec![1., 1.]);
    println!("{}", u.dot(v));
    // 0.0
    let mut u = Vector::from(vec![1., 1.]);
    let v = Vector::from(vec![1., 1.]);
    println!("{}", u.dot(v));
    // 2.0
    let mut u = Vector::from(vec![-1., 6.]);
    let v = Vector::from(vec![3., 2.]);
    println!("{}", u.dot(v));
    // 9.0
}