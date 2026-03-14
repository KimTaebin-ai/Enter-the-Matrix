use crate::utils::{Operations};
use std::ops::{Add, Sub, Mul, Div, Neg, 
    AddAssign, SubAssign, MulAssign, DivAssign};
use std::fmt;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Complex {
    pub re: f32,
    pub im: f32,
}

impl Complex {
    pub fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }

    pub fn mul_add(self, a: Self, b: Self) -> Self {
        <Self as Operations>::fma(self, a, b)
    }
}

// 가우스 소거법 피버팅을 위해 PartialOrd 구현
impl PartialOrd for Complex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // 복소수의 '크기'인 re^2 + im^2를 기준으로 비교
        let self_mag = self.re * self.re + self.im * self.im;
        let other_mag = other.re * other.re + other.im * other.im;
        self_mag.partial_cmp(&other_mag)
    }
}

impl Add for Complex { 
    type Output = Self; 
    fn add(self, rhs: Self) -> Self { 
        Complex::new(self.re + rhs.re, self.im + rhs.im) 
    } 
}

impl Sub for Complex { 
    type Output = Self; 
    fn sub(self, rhs: Self) -> Self { 
        Complex::new(self.re - rhs.re, self.im - rhs.im) 
    } 
}

impl Mul for Complex { 
    type Output = Self; 
    fn mul(self, rhs: Self) -> Self { 
        Complex::new(self.re * rhs.re - self.im * rhs.im, self.re * rhs.im + self.im * rhs.re) 
    } 
}

impl Div for Complex { 
    type Output = Self; 
    fn div(self, rhs: Self) -> Self {
        let den = rhs.re * rhs.re + rhs.im * rhs.im;
        Complex::new((self.re * rhs.re + self.im * rhs.im) / den, (self.im * rhs.re - self.re * rhs.im) / den)
    }
}

impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let is_re_int = self.re.fract() == 0.0;
        let is_im_int = self.im.fract() == 0.0;
        
        let re_str = if is_re_int {
            format!("{:.1}", self.re)
        } else {
            format!("{}", self.re)
        };

        let im_str = if is_im_int {
            format!("{:.1}", self.im)
        } else {
            format!("{}", self.im)
        };

        write!(f, "{} + {}i", re_str, im_str)
    }
}



impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        self.re -= rhs.re;
        self.im -= rhs.im;
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        let re = self.re * rhs.re - self.im * rhs.im;
        let im = self.re * rhs.im + self.im * rhs.re;
        self.re = re;
        self.im = im;
    }
}

impl DivAssign for Complex {
    fn div_assign(&mut self, rhs: Self) {
        let den = rhs.re * rhs.re + rhs.im * rhs.im;
        let re = (self.re * rhs.re + self.im * rhs.im) / den;
        let im = (self.im * rhs.re - self.re * rhs.im) / den;
        self.re = re;
        self.im = im;
    }
}