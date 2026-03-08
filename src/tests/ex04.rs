/**
 * Exercise 04 - Norm
 * 
 * Allowed mathematical functions : fused multiply-add function, pow, max
 * 
 * - Maximum time complexity : O(n)
 * - Maximum space complexity : O(n)
 */


use crate::utils::{Vector};

pub fn ex04() {
    let mut u = Vector::from(vec![0., 0., 0.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 0.0, 0.0, 0.0
    let mut u = Vector::from(vec![1., 2., 3.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 6.0, 3.74165738, 3.0
    let mut u = Vector::from(vec![-1., -2.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 3.0, 2.236067977, 2.0
}