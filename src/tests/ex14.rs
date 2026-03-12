/**
 * Exercise 14 - Bonus: Projection matrix
 * 
 * Allowed mathematical functions : tan, fused multiply-add function
 * 
 * - Maximum time complexity : N/A
 * - Maximum space complexity : N/A
 */

use crate::utils::{Matrix};

pub fn ex14() {
    let fov = 90.0;          // 시야각 (Degree)
    let ratio = 1.0;         // 가로세로비 (1:1)
    let near = 0.1;          // 근평면
    let far = 100.0;         // 원평면

    let projection_matrix = Matrix::projection(fov, ratio, near, far);

    for (i, row) in projection_matrix.data.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            if j < row.len() - 1 {
                print!("{:.3}, ", val);
            } else {
                print!("{:.3}", val);
            }
        }
        if i < projection_matrix.data.len() {
            println!();
        }
    }
}