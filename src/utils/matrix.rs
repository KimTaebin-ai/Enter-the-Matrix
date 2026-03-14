use std::ops::{Add, AddAssign, MulAssign, Neg, SubAssign};
use std::ops::{Sub, Mul, Div};
use std::fmt;
use crate::utils::Operations;
use crate::utils::vector::DisplayScalar;

use super::{Vector, Lerp};

#[derive(Clone)]
pub struct Matrix<K> {
    pub(crate) data: Vec<Vec<K>>,
}

impl<K> fmt::Display for Matrix<K>
where 
    K: Operations
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            write!(f, "[")?;
            for (i, val) in row.iter().enumerate() {
                write!(f, "{}", val.fmt_precision())?;

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

impl<K, const R: usize, const C: usize> From<[[K; C]; R]> for Matrix<K> 
where K: Clone
{
    fn from(data: [[K; C]; R]) -> Self {
        let mut res_data = Vec::with_capacity(R);
        for row in data.into_iter() {
            res_data.push(row.to_vec());
        }
        Matrix { data: res_data }
    }
}

impl<K> From<Vec<Vec<K>>> for Matrix<K> {
    fn from(data: Vec<Vec<K>>) -> Self {
        Matrix { data }
    }
}

impl<K> Matrix<K> 
where K: AddAssign + SubAssign + MulAssign + Copy {
    pub fn add(&mut self, v: Matrix<K>) {
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
    
    pub fn sub(&mut self, v: Matrix<K>) {
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

impl<K> Lerp<f32> for Matrix<K> 
where 
    K: Copy + Default + Operations + 
    AddAssign<K> + SubAssign<K> + MulAssign<K> + 
    Sub<Output = K> + Lerp<f32>,
    Vector<K>: Lerp<f32>
{
    fn lerp(u: Self, v: Self, t: f32) -> Self {
        let u_shape = u.shape();
        let v_shape = v.shape();

        if u_shape != v_shape {
            panic!(
                "Matrix shapes must match for lerp: {:?} != {:?}", 
                u_shape, v_shape
            );
        }

        let res_rows = u.data.into_iter()
            .zip(v.data.into_iter())
            .map(|(row_u, row_v)| {
                let u_vec = Vector::from(row_u);
                let v_vec = Vector::from(row_v);
                
                let lerped_vec = Vector::lerp(u_vec, v_vec, t);
                lerped_vec.data
            })
            .collect();

        Matrix { data: res_rows }
    }
}

impl<K> Matrix<K> 
where K: Default + Clone + Operations + Copy + 
fmt::Display +
AddAssign + MulAssign + SubAssign + fmt::Debug 
{
    // Matrix * Vector = Vector
    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
        let (rows, cols) = self.shape();
        let vec_size = vec.size();

        // 행렬의 열 개수 == 벡터의 크기
        if cols != vec_size {
            panic!(
                "Invalid dimensions for Matrix-Vector multiplication: Matrix({}x{}), Vector({})",
                rows, cols, vec_size
            );
        }

        let result_data = self.data.iter()
            .map(|row_data| {
                // 행 데이터 -> 임시 벡터
                let row_vec = Vector::from(row_data.clone()); 
                // 행 벡터와 입력 벡터의 내적 수행
                row_vec.dot(vec).0
            })
            .collect();

        Vector::from(result_data)
    }

    // Matrix * Matrix = Matrix
    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K> {
        let (rows_a, cols_a) = self.shape();
        let (rows_b, cols_b) = mat.shape();

        // A의 열 == B의 행
        if cols_a != rows_b {
            panic!(
                "Matrix shapes incompatible for multiplication: ({}x{}) * ({}x{})", 
                rows_a, cols_a, rows_b, cols_b
            );
        }

        // 행렬 B를 전치
        let mat_t = mat.transpose();

        let res_data: Vec<Vec<K>> = self.data.iter()
            .map(|row_a_data| {
                let row_a_vec = Vector::from(row_a_data.clone());
                
                // 전치된 B의 각 행(원래 B의 열)과 내적 수행
                mat_t.data.iter()
                    .map(|row_b_t_data| {
                        let row_b_t_vec = Vector::from(row_b_t_data.clone());
                        row_a_vec.dot(&row_b_t_vec).0 // 내적 결과 스칼라 추출
                    })
                    .collect::<Vec<K>>()
            })
            .collect();

        Matrix::from(res_data)
    }

    pub fn trace(&self) -> DisplayScalar<K> {
        if !self.is_square() {
            panic!("Trace is only defined for square matrices.");
        }

        let sum = self.data.iter()
            .enumerate()
            .fold(K::default(), |mut acc, (i, row)| {
                acc += row[i];
                acc
            });

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

    pub fn identity(size: usize) -> Self {
        let mut data = vec![vec![K::default(); size]; size];
        let one = K::from_f32(1.0);
        for i in 0..size {
            data[i][i] = one;
        }
        Matrix { data }
    }
}

impl<K> Matrix<K> 
where 
    K: Copy + Default + Operations + PartialOrd + fmt::Display + fmt::Debug +
       AddAssign + MulAssign + SubAssign +
       Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Div<Output = K> + Neg<Output = K>
{
    fn gauss_jordan_engine(&self, augmented: Option<&Matrix<K>>) -> (Vec<Vec<K>>, Option<Vec<Vec<K>>>, i32, K) {
        let mut data = self.data.clone();
        let mut aug_data = augmented.map(|m| m.data.clone());
        let (rows, cols) = self.shape();
        
        let mut pivot_row = 0;
        let mut pivot_col = 0;
        let mut swap_count = 0;
        
        // 1.0 대용값 생성 (K가 f32인 경우 1.0, 제네릭인 경우 별도 처리 필요)
        let mut pivot_product = K::from_f32(1.0); 
        let epsilon = K::from_f32(1e-9);

        while pivot_row < rows && pivot_col < cols {
            // Partial Pivoting 현재 열에서 절댓값이 가장 큰 행 선택
            let mut max_idx = pivot_row;
            for i in pivot_row + 1..rows {
                if data[i][pivot_col].abs() > data[max_idx][pivot_col].abs() {
                    max_idx = i;
                }
            }

            // 피벗이 0에 가까우면 해당 열은 건너뜀
            if data[max_idx][pivot_col].abs() < epsilon {
                pivot_product = K::default(); // Singular matrix인 경우 determinant는 0
                pivot_col += 1;
                continue;
            }

            // 행 교환 (Swap)
            if max_idx != pivot_row {
                data.swap(pivot_row, max_idx);
                if let Some(ref mut ad) = aug_data {
                    ad.swap(pivot_row, max_idx);
                }
                swap_count += 1;
            }

            let pivot_val = data[pivot_row][pivot_col];
            pivot_product = pivot_product * pivot_val;

            // 피벗 행 정규화 (Normalization)
            for j in 0..cols {
                data[pivot_row][j] = data[pivot_row][j] / pivot_val;
                if data[pivot_row][j].abs() < epsilon { data[pivot_row][j] = K::default(); }
            }
            if let Some(ref mut ad) = aug_data {
                let aug_cols = ad[0].len();
                for j in 0..aug_cols {
                    ad[pivot_row][j] = ad[pivot_row][j] / pivot_val;
                    if ad[pivot_row][j].abs() < epsilon { ad[pivot_row][j] = K::default(); }
                }
            }

            // Elimination
            for i in 0..rows {
                if i != pivot_row {
                    let factor = data[i][pivot_col];
                    // R_i = R_i - factor * R_pivot
                    for j in 0..cols {
                        data[i][j] = data[i][j] - (factor * data[pivot_row][j]);
                        if data[i][j].abs() < epsilon { data[i][j] = K::default(); }
                    }
                    if let Some(ref mut ad) = aug_data {
                        let aug_cols = ad[0].len();
                        for j in 0..aug_cols {
                            ad[i][j] = ad[i][j] - (factor * ad[pivot_row][j]);
                            if ad[i][j].abs() < epsilon { ad[i][j] = K::default(); }
                        }
                    }
                }
            }

            pivot_row += 1;
            pivot_col += 1;
        }

        (data, aug_data, swap_count, pivot_product)
    }

    pub fn row_echelon(&self) -> Matrix<K> {
        let (res_data, _, _, _) = self.gauss_jordan_engine(None);
        Matrix::from(res_data)
    }

    pub fn determinant(&self) -> DisplayScalar<K> {
        if !self.is_square() { panic!("Matrix must be square"); }
        // 여기서도 두 번째 자리를 비워줍니다.
        let (_, _, swap_count, pivot_prod) = self.gauss_jordan_engine(None);
        
        if swap_count % 2 == 0 { 
            DisplayScalar(pivot_prod)
        } 
        else { 
            DisplayScalar(-pivot_prod)
        }
    }

    pub fn inverse(&self) -> Matrix<K> {
        if !self.is_square() { panic!("Matrix must be square"); }
        let (rows, _) = self.shape();
        let identity = Matrix::identity(rows); // 단위 행렬 생성기 필요
        
        let (_, aug_data, _, pivot_prod) = self.gauss_jordan_engine(Some(&identity));
        
        // Singular Matrix 체크: 피벗의 곱이 0이거나 최종 결과가 단위행렬이 아니면 역행렬 없음
        if pivot_prod.abs() < K::from_f32(1e-9) {
            panic!("Matrix is singular and cannot be inverted.");
        }

        Matrix::from(aug_data.expect("Augmented data should exist"))
    }

    pub fn rank(&self) -> usize {
        let (res_data, _, _, _) = self.gauss_jordan_engine(None);
        
        let epsilon = K::from_f32(1e-9);

        // 소거된 데이터에서 0이 아닌 행(Pivot이 존재하는 행)의 개수를 카운트
        res_data.iter().filter(|row| {
            // 행의 모든 원소 중 하나라도 epsilon보다 크면 0이 아닌 행으로 간주
            row.iter().any(|&val| val.abs() > epsilon)
        }).count()
    }
}

impl Matrix<f32> {
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