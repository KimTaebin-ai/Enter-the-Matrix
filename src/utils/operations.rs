use crate::utils::Complex;

/// f32를 정수면 `"1.0"`, 소수면 `"1.5"` 형식으로 변환
pub(crate) fn fmt_f32(val: f32) -> String {
    if val.fract() == 0.0 { format!("{:.1}", val) } else { format!("{}", val) }
}

pub trait Operations: Sized {
    fn fma(a: Self, b: Self, c: Self) -> Self;
    fn abs(self) -> Self;
    fn from_f32(val: f32) -> Self;
    fn fmt_precision(&self) -> String;
    fn conj(self) -> Self { self }
    fn get_re(self) -> f32;
}

impl Operations for f32 {
    fn fma(a: Self, b: Self, c: Self) -> Self { a.mul_add(b, c) }
    fn abs(self) -> Self { f32::abs(self) }
    fn from_f32(val: f32) -> Self { val }
    fn fmt_precision(&self) -> String { fmt_f32(*self) }
    fn get_re(self) -> f32 { self }
}

impl Operations for Complex {
    fn fma(a: Self, b: Self, c: Self) -> Self {
        Complex::new(
            a.re * b.re - a.im * b.im + c.re,
            a.re * b.im + a.im * b.re + c.im,
        )
    }

    fn abs(self) -> Self {
        Complex::new((self.re * self.re + self.im * self.im).sqrt(), 0.0)
    }

    fn from_f32(val: f32) -> Self { Complex::new(val, 0.0) }

    fn fmt_precision(&self) -> String {
        let re_str = fmt_f32(self.re);
        let im_str = fmt_f32(self.im.abs());
        if self.im >= 0.0 { format!("{} + {}i", re_str, im_str) }
        else               { format!("{} - {}i", re_str, im_str) }
    }

    fn conj(self) -> Self { Complex::new(self.re, -self.im) }
    fn get_re(self) -> f32 { self.re }
}

pub trait Lerp<T> {
    fn lerp(u: Self, v: Self, t: T) -> Self;
}

impl Lerp<f32> for f32 {
    fn lerp(u: Self, v: Self, t: f32) -> Self { t.mul_add(v - u, u) }
}

impl Lerp<f32> for Complex {
    fn lerp(u: Self, v: Self, t: f32) -> Self {
        Complex::from_f32(t).mul_add(v - u, u)
    }
}