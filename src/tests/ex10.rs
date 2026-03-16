/**
 * Exercise 10 - row-echelon form
 * 
 * Allowed mathematical functions : None
 * 
 * - Maximum time complexity : O(n^3)
 * - Maximum space complexity : O(n^2)
 */

use crate::utils::{Matrix};

pub fn ex10() {
    // let u = Matrix::from([
    // [1., 0., 0.],
    // [0., 1., 0.],
    // [0., 0., 1.],
    // ]);
    // println!("{}", u.row_echelon());
    // // [1.0, 0.0, 0.0]
    // // [0.0, 1.0, 0.0]
    // // [0.0, 0.0, 1.0]
    // let u = Matrix::from([
    // [1., 2.],
    // [3., 4.],
    // ]);
    // println!("{}", u.row_echelon());
    // // [1.0, 0.0]
    // // [0.0, 1.0]
    // let u = Matrix::from([
    // [1., 2.],
    // [2., 4.],
    // ]);
    // println!("{}", u.row_echelon());
    // // [1.0, 2.0]
    // // [0.0, 0.0]
    // let u = Matrix::from([
    // [8., 5., -2., 4., 28.],
    // [4., 2.5, 20., 4., -4.],
    // [8., 5., 1., 4., 17.],
    // ]);
    // println!("{}", u.row_echelon());
    // // [1.0, 0.625, 0.0, 0.0, -12.1666667]
    // // [0.0, 0.0, 1.0, 0.0, -3.6666667]
    // // [0.0, 0.0, 0.0, 1.0, 29.5 ]

    // 케이스 1: [[0, 0], [0, 0]] -> 변동 없음
    let u1 = Matrix::from([
        [0., 0.],
        [0., 0.]
    ]);
    println!("Case 1:\n{}", u1.row_echelon());

    // 케이스 2: 단위 행렬 [[1, 0], [0, 1]] -> 변동 없음
    let u2 = Matrix::from([
        [1., 0.],
        [0., 1.]
    ]);
    println!("Case 2:\n{}", u2.row_echelon());

    // 케이스 3: [[4, 2], [2, 1]] -> [[1, 0.5], [0, 0]] (행 간의 종속성 존재)
    let u3 = Matrix::from([
        [4., 2.],
        [2., 1.]
    ]);
    println!("Case 3:\n{}", u3.row_echelon());

    // 케이스 4: [[-7, 2], [4, 8]] -> [[1, 0], [0, 1]] (역행렬이 존재하는 경우)
    let u4 = Matrix::from([
        [-7., 2.],
        [4., 8.]
    ]);
    println!("Case 4:\n{}", u4.row_echelon());

    // 케이스 5: [[1, 2], [4, 8]] -> [[1, 2], [0, 0]] (두 번째 행이 첫 번째 행의 4배)
    let u5 = Matrix::from([
        [1., 2.],
        [4., 8.]
    ]);
    println!("Case 5:\n{}", u5.row_echelon());

}