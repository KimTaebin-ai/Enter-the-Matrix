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
    // let e1 = Vector::from(vec![1., 0., 0.]);
    // let e2 = Vector::from(vec![0., 1., 0.]);
    // let e3 = Vector::from(vec![0., 0., 1.]);
    
    // let v1 = Vector::from(vec![1., 2., 3.]);
    // let v2 = Vector::from(vec![0., 10., -100.]);
    
    // println!("{}", linear_combination(
    //     &vec![e1, e2, e3], 
    //     &vec![10., -2., 0.5]));
    // // [10.0]
    // // [-2.0]
    // // [0.5]
    // println!("{}", linear_combination(
    //     &vec![v1, v2], 
    //     &vec![10., -2.]));
    // // [10.0]
    // // [0.0]
    // // [230.0]

    // 케이스 1: Vector([-42, 42])와 계수 [-1]
    let v1_1 = Vector::from(vec![-42., 42.]);
    println!("Case 1: {}", linear_combination(
        &vec![v1_1], 
        &vec![-1.]
    ));

    // 케이스 2: Vector([-42]), Vector([-42]), Vector([-42])와 계수 [-1, 1, 0]
    let v2_1 = Vector::from(vec![-42.]);
    let v2_2 = Vector::from(vec![-42.]);
    let v2_3 = Vector::from(vec![-42.]);
    println!("Case 2: {}", linear_combination(
        &vec![v2_1, v2_2, v2_3], 
        &vec![-1., 1., 0.]
    ));

    // 케이스 3: Vector([-42, 42]), Vector([1, 3]), Vector([10, 20])와 계수 [1, -10, -1]
    let v3_1 = Vector::from(vec![-42., 42.]);
    let v3_2 = Vector::from(vec![1., 3.]);
    let v3_3 = Vector::from(vec![10., 20.]);
    println!("Case 3: {}", linear_combination(
        &vec![v3_1, v3_2, v3_3], 
        &vec![1., -10., -1.]
    ));

    // 케이스 4: Vector([-42, 100, -69.5]), Vector([1, 3, 5])와 계수 [1, -10]
    let v4_1 = Vector::from(vec![-42., 100., -69.5]);
    let v4_2 = Vector::from(vec![1., 3., 5.]);
    println!("Case 4: {}", linear_combination(
        &vec![v4_1, v4_2], 
        &vec![1., -10.]
    ));
}