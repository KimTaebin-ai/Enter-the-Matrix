/**
 * Exercise 05 - Cosine
 * 
 * Allowed mathematical functions : fused multiply-add function
 * 
 * - Maximum time complexity : O(n)
 * - Maximum space complexity : O(n)
 * 
 * If one or both vectors are zero, the behaviour is undefined.
 * If the vectors are of different sizes, the behavior is undefined
 */

use crate::utils::{Vector};
use crate::utils::vector::angle_cos;

pub fn ex05() {
    let u = Vector::from(vec![1., 0.]);
    let v = Vector::from(vec![1., 0.]);
    println!("{}", angle_cos(&u, &v));
    // 1.0
    let u = Vector::from(vec![1., 0.]);
    let v = Vector::from(vec![0., 1.]);
    println!("{}", angle_cos(&u, &v));
    // 0.0
    let u = Vector::from(vec![-1., 1.]);
    let v = Vector::from(vec![ 1., -1.]);
    println!("{}", angle_cos(&u, &v));
    // -1.0
    let u = Vector::from(vec![2., 1.]);
    let v = Vector::from(vec![4., 2.]);
    println!("{}", angle_cos(&u, &v));
    // 1.0
    let u = Vector::from(vec![1., 2., 3.]);
    let v = Vector::from(vec![4., 5., 6.]);
    println!("{}", angle_cos(&u, &v));
    // 0.974631846
}