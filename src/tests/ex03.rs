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
    // let u = Vector::from(vec![0., 0.]);
    // let v = Vector::from(vec![1., 1.]);
    // println!("{}", u.dot(&v));
    // // 0.0
    // let u = Vector::from(vec![1., 1.]);
    // let v = Vector::from(vec![1., 1.]);
    // println!("{}", u.dot(&v));
    // // 2.0
    // let u = Vector::from(vec![-1., 6.]);
    // let v = Vector::from(vec![3., 2.]);
    // println!("{}", u.dot(&v));
    // // 9.0

    // 케이스 1: [0, 0] · [0, 0] = 0.0
    let u1 = Vector::from(vec![0., 0.]);
    let v1 = Vector::from(vec![0., 0.]);
    println!("Case 1: {:.1}", u1.dot(&v1));

    // 케이스 2: [1, 0] · [0, 0] = 0.0
    let u2 = Vector::from(vec![1., 0.]);
    let v2 = Vector::from(vec![0., 0.]);
    println!("Case 2: {:.1}", u2.dot(&v2));

    // 케이스 3: [1, 0] · [1, 0] = 1.0
    let u3 = Vector::from(vec![1., 0.]);
    let v3 = Vector::from(vec![1., 0.]);
    println!("Case 3: {:.1}", u3.dot(&v3));

    // 케이스 4: [1, 0] · [0, 1] = 0.0 (직교)
    let u4 = Vector::from(vec![1., 0.]);
    let v4 = Vector::from(vec![0., 1.]);
    println!("Case 4: {:.1}", u4.dot(&v4));

    // 케이스 5: [1, 1] · [1, 1] = 2.0
    let u5 = Vector::from(vec![1., 1.]);
    let v5 = Vector::from(vec![1., 1.]);
    println!("Case 5: {:.1}", u5.dot(&v5));

    // 케이스 6: [4, 2] · [2, 1] = 10.0
    let u6 = Vector::from(vec![4., 2.]);
    let v6 = Vector::from(vec![2., 1.]);
    println!("Case 6: {:.1}", u6.dot(&v6));
}