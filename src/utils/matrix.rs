use std::ops::{AddAssign, SubAssign, MulAssign};
use std::fmt;
use super::{Vector, Lerp};

pub struct Matrix<K> {
    pub(crate) data: Vec<Vec<K>>,

    // 여기서 예외처리를 잡아라
}

impl<K> fmt::Display for Matrix<K>
where K: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            write!(f, "[")?;
            for (i, val) in row.iter().enumerate() {
                write!(f, "{:.1}", val)?;
                if i < row.len() - 1 {
                    write!(f, ", ")?;
                }
            }
            writeln!(f, "]")?;
        }
        Ok(())
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

    pub fn add(&mut self, v: Matrix<K>) {
        // if self.data.len() != v.data.len() {
        //     panic!("Vector sizes must match for addition");
        // }

        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                self.data[i][j] += v.data[i][j];
            }
        }
    }
    pub fn sub(&mut self, v: Matrix<K>) {
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
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