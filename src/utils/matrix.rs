use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::utils::Operations;
use crate::utils::vector::DisplayScalar;
use super::{Lerp, Vector};

#[derive(Clone)]
pub struct Matrix<K> {
    pub(crate) data: Vec<Vec<K>>,
}

// ── 표시 ─────────────────────────────────────────────────────────────────────

impl<K: Operations> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            let inner = row.iter()
                .map(|v| v.fmt_precision())
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(f, "[{}]", inner)?;
        }
        Ok(())
    }
}

// ── 생성 / 기본 메서드 ────────────────────────────────────────────────────────

impl<K> Matrix<K> {
    pub fn shape(&self) -> (usize, usize) {
        (self.data.len(), self.data.first().map_or(0, |r| r.len()))
    }

    pub fn is_square(&self) -> bool {
        let (r, c) = self.shape();
        r > 0 && r == c
    }
}

impl<K: Clone, const R: usize, const C: usize> From<[[K; C]; R]> for Matrix<K> {
    fn from(data: [[K; C]; R]) -> Self {
        Matrix { data: data.into_iter().map(|row| row.to_vec()).collect() }
    }
}

impl<K> From<Vec<Vec<K>>> for Matrix<K> {
    fn from(data: Vec<Vec<K>>) -> Self { Matrix { data } }
}

// ── add / sub / scl ───────────────────────────────────────────────────────────

impl<K: AddAssign + SubAssign + MulAssign + Copy> Matrix<K> {
    pub fn add(&mut self, v: Matrix<K>) {
        assert_eq!(self.shape(), v.shape(), "Matrix shapes must match for addition");
        self.data.iter_mut().zip(v.data)
            .for_each(|(ra, rb)| ra.iter_mut().zip(rb).for_each(|(a, b)| *a += b));
    }

    pub fn sub(&mut self, v: Matrix<K>) {
        assert_eq!(self.shape(), v.shape(), "Matrix shapes must match for subtraction");
        self.data.iter_mut().zip(v.data)
            .for_each(|(ra, rb)| ra.iter_mut().zip(rb).for_each(|(a, b)| *a -= b));
    }

    pub fn scl(&mut self, a: K) {
        self.data.iter_mut().flatten().for_each(|v| *v *= a);
    }
}

// ── Lerp ─────────────────────────────────────────────────────────────────────

impl<K> Lerp<f32> for Matrix<K>
where
    K: Copy + Default + Operations + AddAssign + SubAssign + MulAssign + Sub<Output = K> + Lerp<f32>,
    Vector<K>: Lerp<f32>,
{
    fn lerp(u: Self, v: Self, t: f32) -> Self {
        assert_eq!(u.shape(), v.shape(), "Matrix shapes must match for lerp");
        let data = u.data.into_iter().zip(v.data)
            .map(|(ru, rv)| Vector::lerp(Vector::from(ru), Vector::from(rv), t).data)
            .collect();
        Matrix { data }
    }
}

// ── 선형대수 연산 ─────────────────────────────────────────────────────────────

impl<K> Matrix<K>
where
    K: Default + Clone + Copy + Operations + fmt::Display + fmt::Debug
     + AddAssign + SubAssign + MulAssign,
{
    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
        let (rows, cols) = self.shape();
        assert_eq!(cols, vec.size(),
            "Invalid dimensions for Matrix-Vector multiplication: Matrix({}x{}), Vector({})",
            rows, cols, vec.size());
        let data = self.data.iter()
            .map(|row| Vector::from(row.clone()).dot(vec).0)
            .collect();
        Vector::from(data)
    }

    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K> {
        let (ra, ca) = self.shape();
        let (rb, cb) = mat.shape();
        assert_eq!(ca, rb,
            "Matrix shapes incompatible for multiplication: ({}x{}) * ({}x{})", ra, ca, rb, cb);

        let mat_t = mat.transpose();
        let data: Vec<Vec<K>> = self.data.iter()
            .map(|row_a| {
                let va = Vector::from(row_a.clone());
                mat_t.data.iter()
                    .map(|row_b| va.dot(&Vector::from(row_b.clone())).0)
                    .collect::<Vec<K>>()
            })
            .collect();
        Matrix::from(data)
    }

    pub fn trace(&self) -> DisplayScalar<K> {
        assert!(self.is_square(), "Trace is only defined for square matrices");
        let sum = self.data.iter().enumerate()
            .fold(K::default(), |mut acc, (i, row)| { acc += row[i]; acc });
        DisplayScalar(sum)
    }

    pub fn transpose(&self) -> Matrix<K> {
        let (rows, cols) = self.shape();
        let mut result = vec![Vec::with_capacity(rows); cols];
        for (i, row) in self.data.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                result[j].push(val);
            }
            let _ = i;
        }
        Matrix { data: result }
    }

    pub fn zero(rows: usize, cols: usize) -> Self {
        Matrix { data: vec![vec![K::default(); cols]; rows] }
    }

    pub fn identity(size: usize) -> Self {
        let mut data = vec![vec![K::default(); size]; size];
        (0..size).for_each(|i| data[i][i] = K::from_f32(1.0));
        Matrix { data }
    }
}

// ── 가우스-조르당 엔진 + 파생 연산 ───────────────────────────────────────────

impl<K> Matrix<K>
where
    K: Copy + Default + Operations + PartialOrd + fmt::Display + fmt::Debug
     + AddAssign + SubAssign + MulAssign
     + Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Div<Output = K> + Neg<Output = K>,
{
    fn gauss_jordan_engine(&self, augmented: Option<&Matrix<K>>) -> (Vec<Vec<K>>, Option<Vec<Vec<K>>>, i32, K) {
        let mut data = self.data.clone();
        let mut aug = augmented.map(|m| m.data.clone());
        let (rows, cols) = self.shape();
        let (mut pr, mut pc, mut swaps) = (0, 0, 0);
        let mut pivot_prod = K::from_f32(1.0);
        let eps = K::from_f32(1e-9);

        let clamp = |v: K| if v.abs() < eps { K::default() } else { v };

        while pr < rows && pc < cols {
            // Partial Pivoting
            let max_idx = (pr..rows).max_by(|&a, &b|
                data[a][pc].abs().partial_cmp(&data[b][pc].abs()).unwrap()
            ).unwrap();

            if data[max_idx][pc].abs() < eps {
                pivot_prod = K::default();
                pc += 1;
                continue;
            }

            if max_idx != pr {
                data.swap(pr, max_idx);
                aug.as_mut().map(|a| a.swap(pr, max_idx));
                swaps += 1;
            }

            let pv = data[pr][pc];
            pivot_prod = pivot_prod * pv;

            // 정규화
            for j in 0..cols      { data[pr][j] = clamp(data[pr][j] / pv); }
            if let Some(ref mut a) = aug {
                for j in 0..a[0].len() { a[pr][j] = clamp(a[pr][j] / pv); }
            }

            // 소거
            for i in (0..rows).filter(|&i| i != pr) {
                let f = data[i][pc];
                for j in 0..cols      { data[i][j] = clamp(data[i][j] - f * data[pr][j]); }
                if let Some(ref mut a) = aug {
                    for j in 0..a[0].len() { a[i][j] = clamp(a[i][j] - f * a[pr][j]); }
                }
            }

            pr += 1;
            pc += 1;
        }

        (data, aug, swaps, pivot_prod)
    }

    pub fn row_echelon(&self) -> Matrix<K> {
        let (data, ..) = self.gauss_jordan_engine(None);
        Matrix::from(data)
    }

    pub fn determinant(&self) -> DisplayScalar<K> {
        assert!(self.is_square(), "Matrix must be square for determinant");
        let (_, _, swaps, prod) = self.gauss_jordan_engine(None);
        DisplayScalar(if swaps % 2 == 0 { prod } else { -prod })
    }

    pub fn inverse(&self) -> Matrix<K> {
        assert!(self.is_square(), "Matrix must be square for inverse");
        let (rows, _) = self.shape();
        let (_, aug, _, prod) = self.gauss_jordan_engine(Some(&Matrix::identity(rows)));
        assert!(prod.abs() >= K::from_f32(1e-9), "Matrix is singular and cannot be inverted");
        Matrix::from(aug.expect("augmented data must exist"))
    }

    pub fn rank(&self) -> usize {
        let (data, ..) = self.gauss_jordan_engine(None);
        let eps = K::from_f32(1e-9);
        data.iter().filter(|row| row.iter().any(|&v| v.abs() > eps)).count()
    }
}

// ── 투영 행렬 (f32 전용) ─────────────────────────────────────────────────────

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