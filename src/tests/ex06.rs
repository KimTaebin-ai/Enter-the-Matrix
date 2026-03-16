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
    // let u = Vector::from(vec![0., 0., 1.]);
    // let v = Vector::from(vec![1., 0., 0.]);
    // println!("{}", cross_product(&u, &v));
    // // [0.]
    // // [1.]
    // // [0.]
    // let u = Vector::from(vec![1., 2., 3.]);
    // let v = Vector::from(vec![4., 5., 6.]);
    // println!("{}", cross_product(&u, &v));
    // // [-3.]
    // // [6.]
    // // [-3.]
    // let u = Vector::from(vec![4., 2., -3.]);
    // let v = Vector::from(vec![-2., -5., 16.]);
    // println!("{}", cross_product(&u, &v));
    // // [17.]
    // // [-58.]
    // // [-16.]

    // 케이스 1: [0, 0, 0] x [0, 0, 0] = [0, 0, 0]
    let u1 = Vector::from(vec![0., 0., 0.]);
    let v1 = Vector::from(vec![0., 0., 0.]);
    println!("Case 1:\n{}", cross_product(&u1, &v1));

    // 케이스 2: [1, 0, 0] x [0, 0, 0] = [0, 0, 0]
    let u2 = Vector::from(vec![1., 0., 0.]);
    let v2 = Vector::from(vec![0., 0., 0.]);
    println!("Case 2:\n{}", cross_product(&u2, &v2));

    // 케이스 3: [1, 0, 0] x [0, 1, 0] = [0, 0, 1] (표준 기저 벡터 간의 외적)
    let u3 = Vector::from(vec![1., 0., 0.]);
    let v3 = Vector::from(vec![0., 1., 0.]);
    println!("Case 3:\n{}", cross_product(&u3, &v3));

    // 케이스 4: [8, 7, -4] x [3, 2, 1] = [15, -20, -5]
    let u4 = Vector::from(vec![8., 7., -4.]);
    let v4 = Vector::from(vec![3., 2., 1.]);
    println!("Case 4:\n{}", cross_product(&u4, &v4));

    // 케이스 5: [1, 1, 1] x [0, 0, 0] = [0, 0, 0]
    let u5 = Vector::from(vec![1., 1., 1.]);
    let v5 = Vector::from(vec![0., 0., 0.]);
    println!("Case 5:\n{}", cross_product(&u5, &v5));

    // 케이스 6: [1, 1, 1] x [1, 1, 1] = [0, 0, 0] (평행한 벡터 간의 외적)
    let u6 = Vector::from(vec![1., 1., 1.]);
    let v6 = Vector::from(vec![1., 1., 1.]);
    println!("Case 6:\n{}", cross_product(&u6, &v6));
}