// K 라는 타입을 담을 건데, 이건 나중에 정하겠음
// K 는 abstract algebra 로 봤을 때 scalar 의 체로 볼 수 있음.

/**
 * We recommend you to implement some utility functions, such as:
• A function to return the size/shape of a vector/matrix.
• A function to tell if a matrix is square.
• A function to print a vector/matrix on the standard output.
• A function to reshape a vector into a matrix, and vice-versa.
 */

use std::ops::{AddAssign, SubAssign, MulAssign};
use std::fmt;

struct Vector<K> {
    data: Vec<K>,
}

impl<K> fmt::Display for Vector<K>
where K: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for val in &self.data {
            writeln!(f, "[{:.1}]", val)?;
        }
        Ok(())
    }
}

impl<K> Vector<K> 
where K: AddAssign + SubAssign + MulAssign + Copy {
    fn from(data: Vec<K>) -> Self {
        Vector { data }
    }

    pub fn add(&mut self, v: Vector<K>) {
        if self.data.len() != v.data.len() {
            panic!("Vector sizes must match for addition");
        }

        for i in 0..self.data.len() {
            self.data[i] += v.data[i];
        }
    }
    pub fn sub(&mut self, v: Vector<K>) {
        if self.data.len() != v.data.len() {
            panic!("Vector sizes must match for subtraction");
        }

        for i in 0..self.data.len() {
            self.data[i] -= v.data[i];
        }
    }

    pub fn scl(&mut self, a: K) {
        for i in 0..self.data.len() {
            self.data[i] *= a;
        }
    }
}

struct  Matrix<K> {
    data: Vec<Vec<K>>,
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
    fn from<const N:usize, const M:usize>(data: [[K; M]; N]) -> Self {
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

/**
 * Allowed mathematical functions : None
 * 
 * Maximum time complexity : O(n)
 * Maximum space complexity : O(n)
 */
fn main() {
    let mut u = Vector::from(vec![2., 3.]);
    let v = Vector::from(vec![5., 7.]);
    u.add(v);
    println!("{}", u);
    // [7.0]
    // [10.0]
    let mut u = Vector::from(vec![2., 3.]);
    let v = Vector::from(vec![5., 7.]);
    u.sub(v);
    println!("{}", u);
    // // [-3.0]
    // // [-4.0]
    let mut u = Vector::from(vec![2., 3.]);
    u.scl(2.);
    println!("{}", u);
    // [4.0]
    // [6.0]

    // ------------------------------------------

    let mut u = Matrix::from([
        [1., 2.],
        [3., 4.]
    ]);
    let v = Matrix::from([
        [7., 4.],
        [-2., 2.]
    ]);
    u.add(v);
    println!("{}", u);
    // [8.0, 6.0]
    // [1.0, 6.0]
    let mut u = Matrix::from([
        [1., 2.],
        [3., 4.]
    ]);
    let v = Matrix::from([
        [7., 4.],
        [-2., 2.]
    ]);
    u.sub(v);
    println!("{}", u);
    // [-6.0, -2.0]
    // [5.0, 2.0]
    let mut u = Matrix::from([
        [1., 2.],
        [3., 4.]
    ]);
    u.scl(2.);
    println!("{}", u);
    // [2.0, 4.0]
    // [6.0, 8.0]

}
