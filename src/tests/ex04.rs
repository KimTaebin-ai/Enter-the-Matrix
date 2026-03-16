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
    // let u = Vector::from(vec![0., 0., 0.]);
    // println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // // 0.0, 0.0, 0.0
    // let u = Vector::from(vec![1., 2., 3.]);
    // println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // // 6.0, 3.74165738, 3.0
    // let u = Vector::from(vec![-1., -2.]);
    // println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // // 3.0, 2.236067977, 2.0

    // --- L2 Norm (Euclidean Norm) Tests ---
    // 기대값: [2, 1] -> 2.236067977, [4, 2] -> 4.472135955

    let v1 = Vector::from(vec![0.]);
    println!("L2 Case 1: {}", v1.norm());

    let v2 = Vector::from(vec![1.]);
    println!("L2 Case 2: {}", v2.norm());

    let v3 = Vector::from(vec![0., 0.]);
    println!("L2 Case 3: {}", v3.norm());

    let v4 = Vector::from(vec![1., 0.]);
    println!("L2 Case 4: {}", v4.norm());

    let v5 = Vector::from(vec![2., 1.]);
    println!("L2 Case 5: {:.9}", v5.norm());

    let v6 = Vector::from(vec![4., 2.]);
    println!("L2 Case 6: {:.9}", v6.norm());

    let v7 = Vector::from(vec![-4., -2.]);
    println!("L2 Case 7: {:.9}", v7.norm());

    println!("---");

    // --- L1 Norm (Taxicab Norm) Tests ---
    // 기대값: [2, 1] -> 3, [4, 2] -> 6

    let v8 = Vector::from(vec![0.]);
    println!("L1 Case 1: {}", v8.norm_1());

    let v9 = Vector::from(vec![1.]);
    println!("L1 Case 2: {}", v9.norm_1());

    let v10 = Vector::from(vec![0., 0.]);
    println!("L1 Case 3: {}", v10.norm_1());

    let v11 = Vector::from(vec![1., 0.]);
    println!("L1 Case 4: {}", v11.norm_1());

    let v12 = Vector::from(vec![2., 1.]);
    println!("L1 Case 5: {}", v12.norm_1());

    let v13 = Vector::from(vec![4., 2.]);
    println!("L1 Case 6: {}", v13.norm_1());

    let v14 = Vector::from(vec![-4., -2.]);
    println!("L1 Case 7: {}", v14.norm_1());
}