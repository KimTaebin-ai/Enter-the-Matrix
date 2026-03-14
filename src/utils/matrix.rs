use std::ops::{AddAssign, SubAssign, MulAssign};
use std::fmt;
use crate::utils::Operations;
use crate::utils::vector::DisplayScalar;

use super::{Vector, Lerp};

/**
 * We recommend you to implement some utility functions, such as:
• A function to return the size/shape of a vector/matrix.
• A function to tell if a matrix is square.
• A function to print a vector/matrix on the standard output.
• A function to reshape a vector into a matrix, and vice-versa.
 */

#[derive(Clone)]
pub struct Matrix<K> {
    pub(crate) data: Vec<Vec<K>>,
}

impl<K> fmt::Display for Matrix<K>
where 
    K: fmt::Display + Into<f32> + Copy
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            write!(f, "[")?;
            for (i, &val) in row.iter().enumerate() {
                let v = val.into() as f32;
                
                if v == (v as i32) as f32 {
                    write!(f, "{:.1}", v)?;
                } else {
                    write!(f, "{}", v)?;
                }

                if i < row.len() - 1 {
                    write!(f, ", ")?;
                }
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

impl<K> Matrix<K> {
    // 행렬의 크기(Shape) 반환: (rows, cols)
    pub fn shape(&self) -> (usize, usize) {
        let rows = self.data.len();
        let cols = if rows > 0 { self.data[0].len() } else { 0 };
        (rows, cols)
    }

    // 정사각 행렬(Square Matrix) 여부 확인
    pub fn is_square(&self) -> bool {
        let (rows, cols) = self.shape();
        rows > 0 && rows == cols
    }
}

impl<K> Matrix<K> 
where K: AddAssign + SubAssign + MulAssign + Copy {
    pub fn from<const N:usize, const M:usize>(data: [[K; M]; N]) -> Self {
        let mut matrix_data = Vec::with_capacity(N);
        for row in data {
            matrix_data.push(row.to_vec());
        }
        Matrix { data: matrix_data }
    }

    pub fn add(&mut self, v: &Matrix<K>) {
        let self_shape = self.shape();
        let other_shape = v.shape();

        if self_shape != other_shape {
            panic!(
                "Matrix shapes must match for addition: {:?} != {:?}", 
                self_shape, other_shape
            );
        }

        for i in 0..self_shape.0 {
            for j in 0..self_shape.1 {
                self.data[i][j] += v.data[i][j];
            }
        }
    }
    
    pub fn sub(&mut self, v: &Matrix<K>) {
        let self_shape = self.shape();
        let other_shape = v.shape();

        if self_shape != other_shape {
            panic!(
                "Matrix shapes must match for subtraction: {:?} != {:?}", 
                self_shape, other_shape
            );
        }

        for i in 0..self_shape.0 {
            for j in 0..self_shape.1 {
                self.data[i][j] -= v.data[i][j];
            }
        }
    }

    pub fn scl(&mut self, a: K) {
        for row in &mut self.data {
            for val in row {
                *val *= a;
            }
        }
    }
    
}

impl Lerp<f32> for Matrix<f32> {
    fn lerp(u: Self, v: Self, t: f32) -> Self {
        if u.data.len() != v.data.len() {
            panic!("Matrix dimensions must match for lerp");
        }

        let mut res_rows = Vec::with_capacity(u.data.len());

        for (row_u, row_v) in u.data.into_iter().zip(v.data.into_iter()) {
            let u_vec = Vector::from(row_u);
            let v_vec = Vector::from(row_v);
            
            let lerped_vector = Vector::lerp(u_vec, v_vec, t);
            
            res_rows.push(lerped_vector.data); 
        }

        Matrix { data: res_rows }
    }
}

impl<K> Matrix<K> 
where K: Default + Clone + Operations + Copy + AddAssign + MulAssign + SubAssign + std::fmt::Debug {
    // Matrix * Vector = Vector
    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
        if self.data.is_empty() || self.data[0].len() != vec.data.len() {
            panic!("Matrix columns must match vector size");
        }

        let mut result_data = Vec::with_capacity(self.data.len());

        for row in &self.data {
            let mut sum = K::default();
            for (i, &val) in row.iter().enumerate() {
                sum = K::fma(val, vec.data[i], sum);
            }
            result_data.push(sum);
        }

        Vector::from(result_data)
    }

    // Matrix * Matrix = Matrix
    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K> {
        let rows_a = self.data.len();
        let cols_a = if rows_a > 0 { self.data[0].len() } else { 0 };
        let rows_b = mat.data.len();
        let cols_b = if rows_b > 0 { mat.data[0].len() } else { 0 };

        if cols_a != rows_b {
            panic!("Matrix shapes are incompatible for multiplication");
        }

        let mut res_data = Vec::with_capacity(rows_a);

        for i in 0..rows_a {
            let mut new_row = Vec::with_capacity(cols_b);
            for j in 0..cols_b {
                let mut sum = K::default();
                for k in 0..cols_a {
                    sum = K::fma(self.data[i][k], mat.data[k][j], sum);
                }
                new_row.push(sum);
            }
            res_data.push(new_row);
        }

        Matrix { data: res_data }
    }

    pub fn trace(&self) -> DisplayScalar<K> {
        if self.data.is_empty() || self.data[0].is_empty() {
            return DisplayScalar(K::default());
        }

        let mut sum = K::default();
        let size = self.data.len().min(self.data[0].len());

        for i in 0..size {
            sum += self.data[i][i];
        }

        DisplayScalar(sum)
    }

    pub fn transpose(&self) -> Matrix<K> {
        let rows = self.data.len();
        if rows == 0 { return Matrix { data: vec![] }; }
        let cols = self.data[0].len();

        let mut result = vec![Vec::with_capacity(rows); cols];

        for i in 0..rows {
            for j in 0..cols {
                result[j].push(self.data[i][j].clone());
            }
        }

        Matrix { data: result }
    }

    pub fn zero(rows: usize, cols: usize) -> Self {
        let mut data = Vec::with_capacity(rows);
        for _ in 0..rows {
            data.push(vec![K::default(); cols]);
        }
        Matrix { data }
    }
}

impl Matrix<f32> {
    pub fn row_echelon(&self) -> Matrix<f32> {
        let mut res = self.clone();
        let rows = res.data.len();
        if rows == 0 { return res; }
        let cols = res.data[0].len();

        let mut pivot_row = 0;
        let mut pivot_col = 0;

        while pivot_row < rows && pivot_col < cols {
            // 현재 열에서 피벗 후보 찾기 (절댓값이 가장 큰 행 선택)
            let mut i_max = pivot_row;
            for i in pivot_row + 1..rows {
                if res.data[i][pivot_col].abs() > res.data[i_max][pivot_col].abs() {
                    i_max = i;
                }
            }

            // 피벗이 0이면 (또는 아주 작으면) 현재 열은 건너뛰고 다음 열로
            if res.data[i_max][pivot_col].abs() < 1e-9 {
                pivot_col += 1;
                continue;
            }

            // 행 교환 (Swap)
            res.data.swap(pivot_row, i_max);

            // 피벗 행 정규화 (선행 원소를 1로 만들기)
            let divisor = res.data[pivot_row][pivot_col];
            for j in pivot_col..cols {
                res.data[pivot_row][j] /= divisor;
            }

            // 피벗 열의 다른 행들을 모두 0으로 소거 (RREF 과정)
            for i in 0..rows {
                if i != pivot_row {
                    let factor = res.data[i][pivot_col];
                    for j in pivot_col..cols {
                        // R_i = R_i - factor * R_pivot
                        res.data[i][j] -= factor * res.data[pivot_row][j];
                    }
                }
            }

            pivot_row += 1;
            pivot_col += 1;
        }

        res
    }
    
    pub fn determinant(&self) -> DisplayScalar<f32> {
        let rows = self.data.len();
        let cols = if rows > 0 { self.data[0].len() } else { 0 };

        // 행렬식은 정사각 행렬에서만 정의됩니다.
        if rows != cols {
            panic!("Determinant is only defined for square matrices.");
        }
        if rows == 0 { return DisplayScalar(1.0); } // 공집합 행렬의 행렬식은 관습적으로 1

        let mut res = self.clone();
        let mut det = 1.0;
        let mut pivot_row = 0;

        for j in 0..cols {
            if pivot_row >= rows { break; }

            // 피벗 찾기
            let mut i_max = pivot_row;
            for i in pivot_row + 1..rows {
                if res.data[i][j].abs() > res.data[i_max][j].abs() {
                    i_max = i;
                }
            }

            // 피벗이 0이면 행렬식은 0 (부피가 없는 상태)
            if res.data[i_max][j].abs() < 1e-9 {
                return DisplayScalar(0.0);
            }

            // 행 교환 시 부호 반전
            if i_max != pivot_row {
                res.data.swap(pivot_row, i_max);
                det *= -1.0;
            }

            // 주대각선 성분의 곱 추적
            // 대각선 원소를 그대로 곱함
            let pivot_val = res.data[pivot_row][j];
            det *= pivot_val;

            // 아래쪽 행들만 소거
            for i in pivot_row + 1..rows {
                let factor = res.data[i][j] / pivot_val;
                for k in j..cols {
                    res.data[i][k] -= factor * res.data[pivot_row][k];
                }
            }
            pivot_row += 1;
        }

        DisplayScalar(det)
    }

    pub fn inverse(&self) -> Matrix<f32> {
        let n = self.data.len();
        // 정사각 행렬 확인
        if n == 0 || n != self.data[0].len() {
            panic!("Inverse is only defined for square matrices.");
        }

        // 행렬 [A | I] 만들기
        let mut augmented = vec![vec![0.0; 2 * n]; n];
        for i in 0..n {
            for j in 0..n {
                augmented[i][j] = self.data[i][j]; // 왼쪽은 A
            }
            augmented[i][i + n] = 1.0; // 오른쪽은 단위 행렬 I
        }

        // gauss jordan elimination 진행
        for i in 0..n {
            // 피벗 찾기
            let mut pivot = i;
            for k in i + 1..n {
                if augmented[k][i].abs() > augmented[pivot][i].abs() {
                    pivot = k;
                }
            }

            // 피벗이 0이면 역행렬이 존재하지 않음 (Singular matrix)
            if augmented[pivot][i].abs() < 1e-9 {
                panic!("Matrix is singular and cannot be inverted.");
            }

            // 행 교환
            augmented.swap(i, pivot);

            // 피벗 행 정규화 (A[i][i]를 1로 만들기)
            let divisor = augmented[i][i];
            for j in i..2 * n {
                augmented[i][j] /= divisor;
            }

            // 다른 모든 행 소거 (피벗 열을 0으로 만들기)
            for k in 0..n {
                if k != i {
                    let factor = augmented[k][i];
                    for j in i..2 * n {
                        augmented[k][j] = (-factor).mul_add(augmented[i][j], augmented[k][j])
                    }
                }
            }
        }

        // [I | A^-1] 에서 결과 추출
        let mut inv_data = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                inv_data[i][j] = augmented[i][j + n];
            }
        }

        Matrix { data: inv_data }
    }

    pub fn rank(&self) -> usize {
        // 가우스 소거법 수행용
        let mut temp = self.data.clone();
        
        let rows = temp.len();
        if rows == 0 { return 0; }
        let cols = temp[0].len();
        let mut pivot_row = 0;

        for j in 0..cols {
            if pivot_row >= rows {
                break;
            }

            // 현재 열에서 가장 큰 절대값을 가진 행을 찾음 (Partial Pivoting)
            let mut max_val = 0.0;
            let mut max_row = pivot_row;
            for i in pivot_row..rows {
                let abs_val = temp[i][j].abs();
                if abs_val > max_val {
                    max_val = abs_val;
                    max_row = i;
                }
            }

            // 피벗이 0에 가깝다면 (수치적 0), 해당 열은 건너뜀
            if max_val < 1e-9 {
                continue;
            }

            // 현재 피벗 행과 최대값 행을 교체
            temp.swap(pivot_row, max_row);

            // 가우스 소거법 적용: 피벗 아래 행들을 0으로 만듦
            for i in (pivot_row + 1)..rows {
                let factor = temp[i][j] / temp[pivot_row][j];
                for k in j..cols {
                    // FMA 연산 활용: data[i][k] = data[i][k] - factor * data[pivot_row][k]
                    // (a * b + c) 형태를 위해 -factor를 전달
                    temp[i][k] = (-factor).mul_add(temp[pivot_row][k], temp[i][k]);
                }
            }

            // 다음 피벗 행으로 이동
            pivot_row += 1;
        }

        // 최종적으로 생성된 피벗의 개수가 곧 Rank임
        pivot_row
    }

    pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov * std::f32::consts::PI / 360.0).tan();
        let mut m = Matrix::zero(4, 4);

        m.data[0][0] = f / ratio;
        m.data[1][1] = f;
        
        m.data[2][2] = far / (near - far);
        m.data[2][3] = -1.0;
        m.data[3][2] = (far * near) / (near - far);
        
        m
    }
}