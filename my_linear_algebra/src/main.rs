// K 라는 타입을 담을 건데, 이건 나중에 정하겠음
// K 는 abstract algebra 로 봤을 때 scalar 의 체로 볼 수 있음.

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

    // fn add(&mut self, v: &Vector<K>) {
    
    // }

    // fn sub(&mut self, v: &Vector<K>) {

    // }

    // fn scl(&mut self, a: K) {

    // }
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
    // u.add(v);
    println!("{}", u);
    // [7.0]
    // [10.0]
    // let mut u = Vector::from(vec![2., 3.]);
    // let v = Vector::from(vec![5., 7.]);
    // u.sub(v);
    // println!("{}", u);
    // // [-3.0]
    // // [-4.0]
    // let mut u = Vector::from(vec![2., 3.]);
    // u.scl(2.);
    // println!("{}", u);
    // [4.0]
    // [6.0]
}
