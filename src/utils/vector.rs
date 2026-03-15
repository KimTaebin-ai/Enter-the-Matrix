use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};
use super::{Lerp, Operations};

#[derive(Clone, PartialEq, Debug)]
pub struct Vector<K> {
    pub(crate) data: Vec<K>,
}

// ── 표시 ────────────────────────────────────────────────────────────────────

impl<K: Operations> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.iter().try_for_each(|v| writeln!(f, "[{}]", v.fmt_precision()))
    }
}

pub struct DisplayScalar<K>(pub K);

impl<K: Operations> fmt::Display for DisplayScalar<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.fmt_precision())
    }
}

// ── 기본 생성 / 크기 ─────────────────────────────────────────────────────────

impl<K> Vector<K> {
    pub fn from(data: Vec<K>) -> Self { Vector { data } }
    pub fn size(&self) -> usize { self.data.len() }
}

// ── add / sub / scl ───────────────────────────────────────────────────────────

impl<K: AddAssign + SubAssign + MulAssign + Copy> Vector<K> {
    pub fn add(&mut self, v: Vector<K>) {
        assert_eq!(self.size(), v.size(), "Vector sizes must match for addition");
        self.data.iter_mut().zip(v.data).for_each(|(a, b)| *a += b);
    }

    pub fn sub(&mut self, v: Vector<K>) {
        assert_eq!(self.size(), v.size(), "Vector sizes must match for subtraction");
        self.data.iter_mut().zip(v.data).for_each(|(a, b)| *a -= b);
    }

    pub fn scl(&mut self, a: K) {
        self.data.iter_mut().for_each(|v| *v *= a);
    }
}

// ── 선형대수 연산 ─────────────────────────────────────────────────────────────

impl<K> Vector<K>
where
    K: Copy + Default + Operations + fmt::Display + AddAssign + SubAssign + MulAssign,
{
    pub fn dot(&self, v: &Vector<K>) -> DisplayScalar<K> {
        assert_eq!(self.size(), v.size(), "Vector sizes must match for dot product");
        let sum = self.data.iter().zip(&v.data)
            .fold(K::default(), |acc, (&a, &b)| K::fma(a, b.conj(), acc));
        DisplayScalar(sum)
    }

    pub fn norm_1(&self) -> DisplayScalar<K> {
        let sum: f32 = self.data.iter().map(|v| v.abs().get_re()).sum();
        DisplayScalar(K::from_f32(sum))
    }

    pub fn norm(&self) -> DisplayScalar<K> {
        DisplayScalar(K::from_f32(self.dot(self).0.get_re().powf(0.5)))
    }

    pub fn norm_inf(&self) -> DisplayScalar<K> {
        let max = self.data.iter()
            .map(|v| v.abs().get_re())
            .fold(0.0_f32, f32::max);
        DisplayScalar(K::from_f32(max))
    }
}

// ── linear_combination ────────────────────────────────────────────────────────

pub fn linear_combination<K>(vectors: &[Vector<K>], coefs: &[K]) -> Vector<K>
where
    K: Copy + Default + Operations + AddAssign + MulAssign,
{
    assert!(!vectors.is_empty(), "vectors must not be empty");
    assert_eq!(vectors.len(), coefs.len(), "vectors and coefs length must match");

    let size = vectors[0].size();
    assert!(vectors.iter().all(|v| v.size() == size), "all vectors must have the same size");

    let mut result = vec![K::default(); size];
    for (v, &s) in vectors.iter().zip(coefs) {
        result.iter_mut().zip(&v.data)
            .for_each(|(acc, &x)| *acc = K::fma(x, s, *acc));
    }
    Vector::from(result)
}

// ── Lerp ─────────────────────────────────────────────────────────────────────

impl<K> Lerp<f32> for Vector<K>
where
    K: Copy + Default + Operations + AddAssign + SubAssign + MulAssign + Sub<Output = K> + Lerp<f32>,
{
    fn lerp(u: Self, v: Self, t: f32) -> Self {
        assert_eq!(u.size(), v.size(), "Vector sizes must match for lerp");
        let data = u.data.iter().zip(&v.data)
            .map(|(&a, &b)| K::lerp(a, b, t))
            .collect();
        Vector::from(data)
    }
}

// ── angle_cos / cross_product ─────────────────────────────────────────────────

pub fn angle_cos<K>(u: &Vector<K>, v: &Vector<K>) -> DisplayScalar<K>
where
    K: Copy + Default + Operations + PartialOrd + fmt::Display
     + AddAssign + SubAssign + MulAssign
     + Add<Output = K> + Sub<Output = K> + Div<Output = K> + Mul<Output = K>,
{
    let (dot, norm_u, norm_v) = (u.dot(v).0, u.norm().0, v.norm().0);
    if norm_u == K::default() || norm_v == K::default() {
        return DisplayScalar(K::default());
    }
    DisplayScalar(K::from_f32(dot.get_re() / (norm_u.get_re() * norm_v.get_re())))
}

pub fn cross_product<K>(u: &Vector<K>, v: &Vector<K>) -> Vector<K>
where
    K: Copy + Default + Operations + AddAssign + SubAssign + MulAssign
     + Mul<Output = K> + Sub<Output = K> + Neg<Output = K>,
{
    assert!(u.size() == 3 && v.size() == 3, "cross product requires 3D vectors");
    let (d, e) = (&u.data, &v.data);
    Vector::from(vec![
        K::fma(d[1], e[2], -(d[2] * e[1])),
        K::fma(d[2], e[0], -(d[0] * e[2])),
        K::fma(d[0], e[1], -(d[1] * e[0])),
    ])
}