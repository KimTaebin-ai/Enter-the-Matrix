/**
 * Exercise 02 - Linear interpolation
 * 
 * Allowed mathematical functions : fused multiply-add function
 * 
 * - Maximum time complexity : O(n)
 * - Maximum space complexity : O(n)
 */
use crate::utils::{Vector, Matrix, lerp};

pub fn ex02() {
    // println!("{:.1}", lerp(0., 1., 0.));
    // // 0.0
    // println!("{:.1}", lerp(0., 1., 1.));
    // // 1.0
    // println!("{:.1}", lerp(0., 1., 0.5));
    // // 0.5
    // println!("{:.1}", lerp(21., 42., 0.3));
    // // 27.3
    // println!("{:.1}", lerp(Vector::from(vec![2., 1.]), Vector::from(vec![4., 2.]), 0.3));
    // // [2.6]
    // // [1.3]
    // println!("{}", lerp(
    // Matrix::from([[2., 1.], [3., 4.]]), 
    // Matrix::from([[20., 10.], [30., 40.]]), 0.5));
    // // [[11., 5.5]
    // // [16.5, 22.]]

    // 케이스 1: lerp(0., 1., 0.) -> 0.
    println!("{:.1}", lerp(0., 1., 0.));

    // 케이스 2: lerp(0., 1., 1.) -> 1.
    println!("{:.1}", lerp(0., 1., 1.));

    // 케이스 3: lerp(0., 42., 0.5) -> 21.0
    println!("{:.1}", lerp(0., 42., 0.5));

    // 케이스 4: lerp(-42., 42., 0.5) -> 0.0
    println!("{:.1}", lerp(-42., 42., 0.5));

    // 케이스 5: lerp(Vector([-42, 42]), Vector([42, -42]), 0.5) -> [0.0, 0.0]
    let v1 = Vector::from(vec![-42., 42.]);
    let v2 = Vector::from(vec![42., -42.]);
    println!("{}", lerp(v1, v2, 0.5));
}