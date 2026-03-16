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
    // let u = Vector::from(vec![1., 0.]);
    // let v = Vector::from(vec![1., 0.]);
    // println!("{}", angle_cos(&u, &v));
    // // 1.0
    // let u = Vector::from(vec![1., 0.]);
    // let v = Vector::from(vec![0., 1.]);
    // println!("{}", angle_cos(&u, &v));
    // // 0.0
    // let u = Vector::from(vec![-1., 1.]);
    // let v = Vector::from(vec![ 1., -1.]);
    // println!("{}", angle_cos(&u, &v));
    // // -1.0
    // let u = Vector::from(vec![2., 1.]);
    // let v = Vector::from(vec![4., 2.]);
    // println!("{}", angle_cos(&u, &v));
    // // 1.0
    // let u = Vector::from(vec![1., 2., 3.]);
    // let v = Vector::from(vec![4., 5., 6.]);
    // println!("{}", angle_cos(&u, &v));
    // // 0.974631846

    // 케이스 1: [1, 0]과 [0, 1] -> 0.0 (직교)
    let u1 = Vector::from(vec![1., 0.]);
    let v1 = Vector::from(vec![0., 1.]);
    println!("Case 1: {}", angle_cos(&u1, &v1));

    // 케이스 2: [8, 7]과 [3, 2] -> 0.9914542955425437
    let u2 = Vector::from(vec![8., 7.]);
    let v2 = Vector::from(vec![3., 2.]);
    println!("Case 2: {}", angle_cos(&u2, &v2));

    // 케이스 3: [1, 1]과 [1, 1] -> 1.0 (동일 방향)
    // 교환 법칙 테스트를 위해 순서를 바꿔서 호출 (v, u)
    let u3 = Vector::from(vec![1., 1.]);
    let v3 = Vector::from(vec![1., 1.]);
    println!("Case 3: {}", angle_cos(&v3, &u3));

    // 케이스 4: [4, 2]과 [1, 1] -> 0.9486832980505138
    let u4 = Vector::from(vec![4., 2.]);
    let v4 = Vector::from(vec![1., 1.]);
    println!("Case 4: {}", angle_cos(&u4, &v4));

    // 케이스 5: [-7, 3]과 [6, 4] -> -0.5462677805469223
    // 교환 법칙 테스트를 위해 순서를 바꿔서 호출 (v, u)
    let u5 = Vector::from(vec![-7., 3.]);
    let v5 = Vector::from(vec![6., 4.]);
    println!("Case 5: {}", angle_cos(&v5, &u5));
}