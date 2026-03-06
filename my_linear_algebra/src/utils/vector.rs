use std::ops::{Add, AddAssign, MulAssign, SubAssign};
use std::fmt;

pub struct Vector<K> {
    pub(crate) data: Vec<K>,
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

impl<K> Vector<K> {
    pub fn from(data: Vec<K>) -> Self {
        Vector { data }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl<K> Vector<K> 
where K: AddAssign + SubAssign + MulAssign + Copy {
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

pub fn linear_combination<K>(vectors: &[Vector<K>], coefs: &[K]) -> Vector<K>
where K: AddAssign + MulAssign + Copy + Default {
    let size = vectors[0].size();
    let mut result = vec![K::default(); size];

    for (v, &scalar) in vectors.iter().zip(coefs.iter()) {
        if v.size() != size {
            panic!("All vectors must have the same size");
        }
        for i in 0..size {
            let mut tmp = v.data[i];
            tmp *= scalar;
            result[i] += tmp;
        }
    }

    Vector::from(result)
}