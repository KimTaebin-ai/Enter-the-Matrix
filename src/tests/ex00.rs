/**
 * Exercise 00 - Add, Subtract and Scale
 * 
 * Allowed mathematical functions : None
 * 
 * - Maximum time complexity : O(n)
 * - Maximum space complexity : O(n)
 */

use crate::utils::{Vector, Matrix};

pub fn ex00() {
    // {
    //     let mut u = Vector::from(vec![1., 0.]);
    //     let v = Vector::from(vec![0., 1.]);

    //     u.add(v);
    //     println!("{}", u);

    // }
    
    // let mut u = Vector::from(vec![1., 1.]);
    // let v = Vector::from(vec![1., 1.]);

    // u.add(v);
    // println!("{}", u);

    // let mut u = Vector::from(vec![21., 21.]);
    // let v = Vector::from(vec![21., 21.]);

    // u.add(v);
    // println!("{}", u);

    // let mut u = Vector::from(vec![-21., 21.]);
    // let v = Vector::from(vec![21., -21.]);

    // u.add(v);
    // println!("{}", u);

    // {
    //     let mut u = Vector::from(vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9.]);
    //     let v = Vector::from(vec![9., 8., 7., 6., 5., 4., 3., 2., 1., 0.]);

    //     u.add(v);
    //     println!("{}", u);
    // }


    // let mut u = Vector::from(vec![2., 3.]);
    // let v = Vector::from(vec![5., 7.]);
    // u.add(v);
    // println!("{}", u);
    // // [7.0]
    // // [10.0]
    // let mut u = Vector::from(vec![2., 3.]);
    // let v = Vector::from(vec![5., 7.]);
    // u.sub(v);
    // println!("{}", u);
    // // // [-3.0]
    // // // [-4.0]
    // let mut u = Vector::from(vec![2., 3.]);
    // u.scl(2.);
    // println!("{}", u);
    // // [4.0]
    // // [6.0]

    // // ------------------------------------------

    // let mut u1 = Matrix::from([[0., 0.], [0., 0.]]);
    // let v1 = Matrix::from([[0., 0.], [0., 0.]]);
    // u1.add(v1);
    // println!("Case 1: {}", u1);

    // // 케이스 2: [[1, 0], [0, 1]] + [[0, 0], [0, 0]]
    // let mut u2 = Matrix::from([[1., 0.], [0., 1.]]);
    // let v2 = Matrix::from([[0., 0.], [0., 0.]]);
    // u2.add(v2);
    // println!("Case 2: {}", u2);

    // // 케이스 3: [[1, 1], [1, 1]] + [[1, 1], [1, 1]]
    // let mut u3 = Matrix::from([[1., 1.], [1., 1.]]);
    // let v3 = Matrix::from([[1., 1.], [1., 1.]]);
    // u3.add(v3);
    // println!("Case 3: {}", u3);

    // // 케이스 4: [[21, 21], [21, 21]] + [[21, 21], [21, 21]]
    // let mut u4 = Matrix::from([[21., 21.], [21., 21.]]);
    // let v4 = Matrix::from([[21., 21.], [21., 21.]]);
    // u4.add(v4);
    // println!("Case 4: {}", u4);
    // // [8.0, 6.0]
    // // [1.0, 6.0]
    // let mut u = Matrix::from([
    //     [1., 2.],
    //     [3., 4.]
    // ]);
    // let v = Matrix::from([
    //     [7., 4.],
    //     [-2., 2.]
    // ]);
    // u.sub(v);
    // println!("{}", u);
    // // [-6.0, -2.0]
    // // [5.0, 2.0]
    // let mut u = Matrix::from([
    //     [1., 2.],
    //     [3., 4.]
    // ]);
    // u.scl(2.);
    // println!("{}", u);
    // // [2.0, 4.0]
    // // [6.0, 8.0]

    // --- Vector Subtraction Tests ---

    // Case 1: [0, 0] - [0, 0] = [0, 0]
    let mut u_v1 = Vector::from(vec![0., 0.]);
    let v_v1 = Vector::from(vec![0., 0.]);
    u_v1.sub(v_v1);
    println!("Vector Case 1: {}", u_v1);

    // Case 2: [1, 0] - [0, 1] = [1, -1]
    let mut u_v2 = Vector::from(vec![1., 0.]);
    let v_v2 = Vector::from(vec![0., 1.]);
    u_v2.sub(v_v2);
    println!("Vector Case 2: {}", u_v2);

    // Case 3: [1, 1] - [1, 1] = [0, 0]
    let mut u_v3 = Vector::from(vec![1., 1.]);
    let v_v3 = Vector::from(vec![1., 1.]);
    u_v3.sub(v_v3);
    println!("Vector Case 3: {}", u_v3);

    // Case 4: [21, 21] - [21, 21] = [0, 0]
    let mut u_v4 = Vector::from(vec![21., 21.]);
    let v_v4 = Vector::from(vec![21., 21.]);
    u_v4.sub(v_v4);
    println!("Vector Case 4: {}", u_v4);

    // Case 5: [-21, 21] - [21, -21] = [-42, 42]
    let mut u_v5 = Vector::from(vec![-21., 21.]);
    let v_v5 = Vector::from(vec![21., -21.]);
    u_v5.sub(v_v5);
    println!("Vector Case 5: {}", u_v5);

    // Case 6: [0..9] - [9..0] = [-9, -7, -5, -3, -1, 1, 3, 5, 7, 9]
    let mut u_v6 = Vector::from(vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9.]);
    let v_v6 = Vector::from(vec![9., 8., 7., 6., 5., 4., 3., 2., 1., 0.]);
    u_v6.sub(v_v6);
    println!("Vector Case 6: {}", u_v6);


    // --- Matrix Subtraction Tests ---

    // Case 1: [[0, 0], [0, 0]] - [[0, 0], [0, 0]]
    let mut u_m1 = Matrix::from([[0., 0.], [0., 0.]]);
    let v_m1 = Matrix::from([[0., 0.], [0., 0.]]);
    u_m1.sub(v_m1);
    println!("Matrix Case 1: {}", u_m1);

    // Case 2: [[1, 0], [0, 1]] - [[0, 0], [0, 0]]
    let mut u_m2 = Matrix::from([[1., 0.], [0., 1.]]);
    let v_m2 = Matrix::from([[0., 0.], [0., 0.]]);
    u_m2.sub(v_m2);
    println!("Matrix Case 2: {}", u_m2);

    // Case 3: [[1, 1], [1, 1]] - [[1, 1], [1, 1]]
    let mut u_m3 = Matrix::from([[1., 1.], [1., 1.]]);
    let v_m3 = Matrix::from([[1., 1.], [1., 1.]]);
    u_m3.sub(v_m3);
    println!("Matrix Case 3: {}", u_m3);

    // Case 4: [[21, 21], [21, 21]] - [[21, 21], [21, 21]]
    let mut u_m4 = Matrix::from([[21., 21.], [21., 21.]]);
    let v_m4 = Matrix::from([[21., 21.], [21., 21.]]);
    u_m4.sub(v_m4);
    println!("Matrix Case 4: {}", u_m4);

    // --- Vector Scaling Tests ---

    // Case 1: [0, 0] * 1.0 = [0, 0]
    let mut u_v1 = Vector::from(vec![0., 0.]);
    u_v1.scl(1.0);
    println!("Vector Case 1: {}", u_v1);

    // Case 2: [1, 0] * 1.0 = [1, 0]
    let mut u_v2 = Vector::from(vec![1., 0.]);
    u_v2.scl(1.0);
    println!("Vector Case 2: {}", u_v2);

    // Case 3: [1, 1] * 2.0 = [2, 2]
    let mut u_v3 = Vector::from(vec![1., 1.]);
    u_v3.scl(2.0);
    println!("Vector Case 3: {}", u_v3);

    // Case 4: [21, 21] * 2.0 = [42, 42]
    let mut u_v4 = Vector::from(vec![21., 21.]);
    u_v4.scl(2.0);
    println!("Vector Case 4: {}", u_v4);

    // Case 5: [42, 42] * 0.5 = [21, 21]
    let mut u_v5 = Vector::from(vec![42., 42.]);
    u_v5.scl(0.5);
    println!("Vector Case 5: {}", u_v5);


    // --- Matrix Scaling Tests ---

    // Case 1: [[0, 0], [0, 0]] * 0.0 = [[0, 0], [0, 0]]
    let mut u_m1 = Matrix::from([[0., 0.], [0., 0.]]);
    u_m1.scl(0.0);
    println!("Matrix Case 1: {}", u_m1);

    // Case 2: [[1, 0], [0, 1]] * 1.0 = [[1, 0], [0, 1]]
    let mut u_m2 = Matrix::from([[1., 0.], [0., 1.]]);
    u_m2.scl(1.0);
    println!("Matrix Case 2: {}", u_m2);

    // Case 3: [[1, 2], [3, 4]] * 2.0 = [[2, 4], [6, 8]]
    let mut u_m3 = Matrix::from([[1., 2.], [3., 4.]]);
    u_m3.scl(2.0);
    println!("Matrix Case 3: {}", u_m3);

    // Case 4: [[21, 21], [21, 21]] * 0.5 = [[10.5, 10.5], [10.5, 10.5]]
    let mut u_m4 = Matrix::from([[21., 21.], [21., 21.]]);
    u_m4.scl(0.5);
    println!("Matrix Case 4: {}", u_m4);
}