/**
 * Exercise 06 - Cross product
 * 
 * Allowed mathematical functions : fused multiply-add function
 * 
 * - Maximum time complexity : N/A
 * - Maximum space complexity : N/A
 * 
 * If one or both vectors are zero, the behaviour is undefined.
 * If the vectors are of different sizes, the behavior is undefined
 */

use crate::utils::{Vector};
use crate::utils::vector::cross_product;

pub fn ex06() {
    let u = Vector::from(vec![0., 0., 1.]);
    let v = Vector::from(vec![1., 0., 0.]);
    println!("{}", cross_product(&u, &v));
    // [0.]
    // [1.]
    // [0.]
    let u = Vector::from(vec![1., 2., 3.]);
    let v = Vector::from(vec![4., 5., 6.]);
    println!("{}", cross_product(&u, &v));
    // [-3.]
    // [6.]
    // [-3.]
    let u = Vector::from(vec![4., 2., -3.]);
    let v = Vector::from(vec![-2., -5., 16.]);
    println!("{}", cross_product(&u, &v));
    // [17.]
    // [-58.]
    // [-16.]
}