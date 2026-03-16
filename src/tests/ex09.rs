/**
 * Exercise 09 - Transpose
 * 
 * Allowed mathematical functions : None
 * 
 * - Maximum time complexity : O(nm)
 * - Maximum space complexity : O(nm)
 */

use crate::utils::{Matrix};

pub fn ex09() {
    // let u = Matrix::from([
    //     [1., 2.], 
    //     [3., 4.]
    // ]);
    // println!("{}", u.transpose());
    // // [1., 3.]
    // // [2., 4.]
    // let u = Matrix::from([
    //     [1., 2.], 
    //     [3., 4.], 
    //     [5., 6.]
    // ]);
    // println!("{}", u.transpose());
    // // [1., 3., 5.]
    // // [2., 4., 6.]

    // 케이스 1: 2x2 영행렬 전치
    let u1 = Matrix::from([
        [0., 0.],
        [0., 0.]
    ]);
    println!("Case 1:\n{}", u1.transpose());

    // 케이스 2: 2x2 단위 행렬 전치 (자기 자신과 동일)
    let u2 = Matrix::from([
        [1., 0.],
        [0., 1.]
    ]);
    println!("Case 2:\n{}", u2.transpose());

    // 케이스 3: 2x2 일반 행렬 [[1, 2], [3, 4]] -> [[1, 3], [2, 4]]
    let u3 = Matrix::from([
        [1., 2.],
        [3., 4.]
    ]);
    println!("Case 3:\n{}", u3.transpose());

    // 케이스 4: 3x3 단위 행렬 전치
    // Matrix::from이 가변 인자를 지원하는 방식(Vec 등)에 맞춰 작성하세요.
    let u4 = Matrix::from(vec![
        vec![1., 0., 0.],
        vec![0., 1., 0.],
        vec![0., 0., 1.],
    ]);
    println!("Case 4:\n{}", u4.transpose());

    // 케이스 5: 3x2 행렬 -> 2x3 행렬 변환
    // [[1, 2], [3, 4], [5, 6]] -> [[1, 3, 5], [2, 4, 6]]
    let u5 = Matrix::from(vec![
        vec![1., 2.],
        vec![3., 4.],
        vec![5., 6.],
    ]);
    println!("Case 5:\n{}", u5.transpose());
}