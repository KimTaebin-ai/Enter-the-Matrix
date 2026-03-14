use crate::utils::Operations;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Complex {
    pub re: f32,
    pub im: f32,
}

impl Complex {
    pub fn new(re: f32, im: f32) -> Self { Self { re, im } }

    pub fn mul_add(self, a: Self, b: Self) -> Self { <Self as Operations>::fma(self, a, b) }

    fn mul_parts(a: Self, b: Self) -> (f32, f32) {
        (a.re * b.re - a.im * b.im, a.re * b.im + a.im * b.re)
    }

    fn div_parts(a: Self, b: Self) -> (f32, f32) {
        let den = b.re * b.re + b.im * b.im;
        ((a.re * b.re + a.im * b.im) / den, (a.im * b.re - a.re * b.im) / den)
    }
}

// 피버팅용: 크기(magnitude) 기준으로 비교
impl PartialOrd for Complex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mag = |c: &Self| c.re * c.re + c.im * c.im;
        mag(self).partial_cmp(&mag(other))
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fmt_precision())
    }
}

impl Add     for Complex { type Output = Self; fn add(self, r: Self) -> Self { Self::new(self.re + r.re, self.im + r.im) } }
impl Sub     for Complex { type Output = Self; fn sub(self, r: Self) -> Self { Self::new(self.re - r.re, self.im - r.im) } }
impl Neg     for Complex { type Output = Self; fn neg(self)          -> Self { Self::new(-self.re, -self.im) } }
impl Mul     for Complex { type Output = Self; fn mul(self, r: Self) -> Self { let (re, im) = Self::mul_parts(self, r); Self::new(re, im) } }
impl Div     for Complex { type Output = Self; fn div(self, r: Self) -> Self { let (re, im) = Self::div_parts(self, r); Self::new(re, im) } }

impl AddAssign for Complex { fn add_assign(&mut self, r: Self) { self.re += r.re; self.im += r.im; } }
impl SubAssign for Complex { fn sub_assign(&mut self, r: Self) { self.re -= r.re; self.im -= r.im; } }
impl MulAssign for Complex { fn mul_assign(&mut self, r: Self) { let (re, im) = Self::mul_parts(*self, r); (self.re, self.im) = (re, im); } }
impl DivAssign for Complex { fn div_assign(&mut self, r: Self) { let (re, im) = Self::div_parts(*self, r); (self.re, self.im) = (re, im); } }