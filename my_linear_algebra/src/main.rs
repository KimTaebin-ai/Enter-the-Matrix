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

// impl<K> Matrix<K> {
//     fn add(&mut self, v: &Matrix<K>);
//     fn sub(&mut self, v: &Matrix<K>);
//     fn scl(&mut self, a: K);
// }


// struct  Matrix<K> {
//     data: Vec<Vec<K>>,
// }



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
}
