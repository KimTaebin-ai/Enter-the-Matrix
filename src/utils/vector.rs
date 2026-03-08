use std::ops::{AddAssign, MulAssign, SubAssign};
use std::fmt;
use super::Operations;
use super::Lerp;

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

pub struct DisplayScalar<K>(pub K);

impl<K: fmt::Display> fmt::Display for DisplayScalar<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}

impl<K> Vector<K> 
where K: AddAssign + SubAssign + MulAssign + Copy {
    pub fn from(data: Vec<K>) -> Self {
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


impl<K> Vector<K>
where K: AddAssign + SubAssign + MulAssign + Copy + Default + Operations + fmt::Display {
    pub fn dot(&self, v: Vector<K>) -> DisplayScalar<K> {
        if self.data.len() != v.data.len() {
            panic!("Vector sizes must match for dot product");
        }

        let mut sum = K::default();
        for i in 0..self.data.len() {
            sum = K::fma(self.data[i], v.data[i], sum);
        }

        DisplayScalar(sum)
    }
}

pub fn linear_combination<K>(vectors: &[Vector<K>], coefs: &[K]) -> Vector<K>
where K: Copy + Default + Operations + AddAssign + SubAssign + MulAssign {
    if vectors.is_empty() || coefs.is_empty() {
        return Vector { data: vec![] };
    }

    let size = vectors[0].data.len();

    let mut result = Vec::with_capacity(size);

    for i in 0..size {
        let mut sum = K::default();
        for (v, &scalar) in vectors.iter().zip(coefs.iter()) {
            sum = K::fma(v.data[i], scalar, sum);
        }
        result.push(sum);
    }

    Vector::from(result)
}

impl Lerp<f32> for Vector<f32> {
    fn lerp(u: Self, v: Self, t: f32) -> Self {
        let mut data = Vec::with_capacity(u.data.len());
        for i in 0..u.data.len() {
            data.push(f32::lerp(u.data[i], v.data[i], t));
        }
        Vector::from(data)
    }
}