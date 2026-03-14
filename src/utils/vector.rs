use std::ops::{AddAssign, MulAssign, SubAssign};
use std::ops::{Add, Sub, Mul, Div, Neg};
use std::fmt;
use super::Operations;
use super::Lerp;
#[derive(Clone)]
pub struct Vector<K> {
    pub(crate) data: Vec<K>,
}

impl<K> fmt::Display for Vector<K>
where K: fmt::Display + Operations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for val in &self.data {
            writeln!(f, "[{}]", val.fmt_precision())?;
        }
        Ok(())
    }
}

pub struct DisplayScalar<K>(pub K);

impl<K> fmt::Display for DisplayScalar<K> 
where 
    K: Operations 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.fmt_precision())
    }
}

impl<K> Vector<K> {
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
where K: Copy + Default + 
        Operations + fmt::Display +
        AddAssign + SubAssign + MulAssign +
{
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
            sum = K::fma(self.data[i], v.data[i].conj(), sum);
        }

        DisplayScalar(sum)
    }

    pub fn norm_1(&self) -> DisplayScalar<K> {
        let mut sum = 0.0;
        for val in &self.data {
            sum += val.abs().get_re(); 
        }
        DisplayScalar(K::from_f32(sum))
    }

    pub fn norm(&self) -> DisplayScalar<K> {
        let dot_res = self.dot(self);
        let dot_val = dot_res.0.get_re();
        DisplayScalar(K::from_f32(dot_val.sqrt()))
    }

    pub fn norm_inf(&self) -> DisplayScalar<K> {
        let mut max_val = 0.0;
        for val in &self.data {
            let mag = val.abs().get_re();
            if mag > max_val { max_val = mag; }
        }
        DisplayScalar(K::from_f32(max_val))
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

impl<K> Lerp<f32> for Vector<K> 
where 
    K: Copy + Default + Operations + 
    AddAssign + SubAssign + MulAssign + 
    Sub<Output = K> + Lerp<f32>
{
    fn lerp(u: Self, v: Self, t: f32) -> Self {
        let size = u.size();
        
        if size != v.size() {
            panic!("Vector sizes must match for lerp: {} != {}", size, v.size());
        }

        let mut res_data = Vec::with_capacity(size);

        for i in 0..size {
            // 선형 보간 수식: u + t * (v - u)
            // K::fma(t, v - u, u)
            let lerped = K::lerp(u.data[i], v.data[i], t);
            res_data.push(lerped);
        }

        Vector::from(res_data)
    }
}

pub fn angle_cos<K>(u: &Vector<K>, v: &Vector<K>) -> DisplayScalar<K> 
where 
    K: Copy + Default + Operations + PartialOrd + fmt::Display +
    AddAssign + SubAssign + MulAssign +
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
    let res = K::from_f32(dot_prod.get_re() / (norm_u.get_re() * norm_v.get_re()));

    DisplayScalar(res)
}

pub fn cross_product<K>(u: &Vector<K>, v: &Vector<K>) -> Vector<K> 
where 
    K: Copy + Default + Operations + 
    AddAssign + SubAssign + MulAssign +
    Mul<Output = K> + Sub<Output = K> + Neg<Output = K>
{
    if u.size() != 3 || v.size() != 3 {
        panic!(
            "Cross product is only defined for 3D vectors: u.size={}, v.size={}", 
            u.size(), v.size()
        );
    }

    let u_d = &u.data;
    let v_d = &v.data;

    // 공식: (u2*v3 - u3*v2, u3*v1 - u1*v3, u1*v2 - u2*v1)   
    let x = K::fma(u_d[1], v_d[2], -(u_d[2] * v_d[1]));
    let y = K::fma(u_d[2], v_d[0], -(u_d[0] * v_d[2]));
    let z = K::fma(u_d[0], v_d[1], -(u_d[1] * v_d[0]));

    Vector::from(vec![x, y, z])
}