use std::ops::{AddAssign, MulAssign, SubAssign};
use std::ops::{Add, Sub, Mul, Div};
use std::fmt;
use super::Operations;
use super::Lerp;
#[derive(Clone)]
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

impl fmt::Display for DisplayScalar<f32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let i = self.0 as i32;
        if self.0 == (i as f32) {
            write!(f, "{:.1}", self.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl<K> Vector<K> {
    // 벡터의 크기(Size) 반환
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl<K> Vector<K> 
where K: AddAssign + SubAssign + MulAssign + Copy {
    pub fn from(data: Vec<K>) -> Self {
        Vector { data }
    }
    
    pub fn add(&mut self, v: Vector<K>) {
        let self_size = self.size();
        let other_size = v.size();

        if self_size != other_size {
            panic!(
                "Vector sizes must match for addition: {} != {}", 
                self_size, other_size
            );
        }

        for i in 0..self_size {
            self.data[i] += v.data[i];
        }
    }

    pub fn sub(&mut self, v: Vector<K>) {
        let self_size = self.size();
        let other_size = v.size();

        if self_size != other_size {
            panic!(
                "Vector sizes must match for subtraction: {} != {}", 
                self_size, other_size
            );
        }

        for i in 0..self_size {
            self.data[i] -= v.data[i];
        }
    }

    pub fn scl(&mut self, a: K) {
        let self_size = self.size();
        for i in 0..self_size {
            self.data[i] *= a;
        }
    }

}


impl<K> Vector<K>
where K: AddAssign + SubAssign + MulAssign + Copy + Default + Operations + fmt::Display {
    pub fn dot(&self, v: &Vector<K>) -> DisplayScalar<K> {
        let self_size = self.size();
        let other_size = v.size();

        if self_size != other_size {
            panic!(
                "Vector sizes must match for dot product: {} != {}", 
                self_size, other_size
            );
        }

        let mut sum = K::default();
        for i in 0..self_size {
            // sum = self[i] * v[i] + sum
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

    if vectors.len() != coefs.len() {
        panic!(
            "Number of vectors and coefficients must match: {} != {}", 
            vectors.len(), coefs.len()
        );
    }

    let vector_size = vectors[0].size();

    if !vectors.iter().all(|v| v.size() == vector_size) {
        panic!("All vectors in linear_combination must have the same size");
    }

    let mut result_data = vec![K::default(); vector_size];

    for (v, &scalar) in vectors.iter().zip(coefs.iter()) {
        for i in 0..vector_size {
            // result[i] = v.data[i] * scalar + result[i]
            result_data[i] = K::fma(v.data[i], scalar, result_data[i]);
        }
    }

    Vector::from(result_data)
}

impl<K> Lerp<K> for Vector<K> 
where 
    K: Copy + Default + Operations + AddAssign + SubAssign + MulAssign + Sub<Output = K>
{
    fn lerp(u: Self, v: Self, t: K) -> Self {
        let size = u.size();
        
        if size != v.size() {
            panic!("Vector sizes must match for lerp: {} != {}", size, v.size());
        }

        let mut res_data = Vec::with_capacity(size);

        for i in 0..size {
            // 선형 보간 수식: u + t * (v - u)
            // K::fma(t, v - u, u)
            let diff = v.data[i] - u.data[i];
            let lerped = K::fma(t, diff, u.data[i]);
            res_data.push(lerped);
        }

        Vector::from(res_data)
    }
}

impl<K> Vector<K>
where 
    K: Copy + Default + Operations + PartialOrd + Add<Output = K> + Sub<Output = K>
{
    pub fn norm_1(&self) -> DisplayScalar<K> {
        let mut res = K::default();
        for &val in &self.data {
            // res = 1 * |val| + res
            res = K::fma(K::default(), K::default(), res + val.abs());
        }
        DisplayScalar(res)
    }

    pub fn norm(&self) -> DisplayScalar<K> {
        let mut squared_sum = K::default();
        for &val in &self.data {
            // squared_sum = val * val + squared_sum
            squared_sum = K::fma(val, val, squared_sum);
        }
        DisplayScalar(squared_sum.sqrt())
    }

    pub fn norm_inf(&self) -> DisplayScalar<K> {
        let mut res = K::default();
        for &val in &self.data {
            let abs_val = val.abs();
            if abs_val > res {
                res = abs_val;
            }
        }
        DisplayScalar(res)
    }
}

pub fn angle_cos<K>(u: &Vector<K>, v: &Vector<K>) -> DisplayScalar<K> 
where 
    K: Copy + Default + Operations + PartialOrd +
    AddAssign + SubAssign + MulAssign +
    std::fmt::Display +
    Add<Output = K> + Sub<Output = K> + Div<Output = K> + Mul<Output = K>
{
    let dot_prod = u.dot(v).0;

    let norm_u = u.norm().0;
    let norm_v = v.norm().0;

    // norm이 0이면 분모가 0이 되어버림
    let zero = K::default();
    if norm_u == zero || norm_v == zero {
        return DisplayScalar(zero);
    }

    // 코사인 값 도출: (u · v) / (||u|| * ||v||)
    let res = dot_prod / (norm_u * norm_v);

    DisplayScalar(res)
}

pub fn cross_product(u: &Vector<f32>, v: &Vector<f32>) -> Vector<f32> {
    if u.data.len() != 3 || v.data.len() != 3 {
        panic!("Cross product is only defined for 3D vectors.");
    }

    let u_d = &u.data;
    let v_d = &v.data;

    let x = u_d[1].mul_add(v_d[2], -(u_d[2] * v_d[1]));
    let y = u_d[2].mul_add(v_d[0], -(u_d[0] * v_d[2]));
    let z = u_d[0].mul_add(v_d[1], -(u_d[1] * v_d[0]));

    Vector::from(vec![x, y, z])
}