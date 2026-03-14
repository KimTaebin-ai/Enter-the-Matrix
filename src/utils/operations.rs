use crate::utils::Complex;

pub trait Operations: Sized {
    fn fma(a: Self, b: Self, c: Self) -> Self;
    fn abs(self) -> Self;
    fn sqrt(self) -> Self;
    fn from_f32(val: f32) -> Self;
    fn fmt_precision(&self) -> String;
    fn conj(self) -> Self;
    fn get_re(self) -> f32;
}

impl Operations for f32 {
    fn fma(a: Self, b: Self, c: Self) -> Self {
        a.mul_add(b, c)
    }

    fn abs(self) -> Self {
        self.abs()
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn from_f32(val: f32) -> Self { val }

    fn fmt_precision(&self) -> String {
        let i = *self as i32;
        if *self == (i as f32) {
            format!("{:.1}", self)
        } else {
            format!("{}", self)
        }
    }

    fn conj(self) -> Self {
        self
    }

    fn get_re(self) -> f32 {
        self
    }
}

impl Operations for Complex {
    fn fma(a: Self, b: Self, c: Self) -> Self {
        // (a.re + a.im*i) * (b.re + b.im*i) + (c.re + c.im*i)
        let re = a.re * b.re - a.im * b.im + c.re;
        let im = a.re * b.im + a.im * b.re + c.im;
        Complex::new(re, im)
    }

    fn abs(self) -> Self {
        // 피버팅 시 비교를 위해 크기(Magnitude)를 실수부에 담아 반환
        Complex::new((self.re * self.re + self.im * self.im).sqrt(), 0.0)
    }

    fn from_f32(val: f32) -> Self {
        Complex::new(val, 0.0)
    }

    fn sqrt(self) -> Self {
        // |z| 계산
        let mag = (self.re * self.re + self.im * self.im).sqrt();
        
        let re = ((mag + self.re) / 2.0).sqrt();
        let im = ((mag - self.re) / 2.0).sqrt();
        
        if self.im < 0.0 {
            Complex::new(re, -im)
        } else {
            Complex::new(re, im)
        }
    }

    // fn fmt_precision(&self) -> String {
    //     format!("{:.1} + {:.1}i", self.re, self.im)
    // }


    fn fmt_precision(&self) -> String {
        let is_re_int = self.re.fract() == 0.0;
        let is_im_int = self.im.fract() == 0.0;

        if is_re_int && is_im_int {
            format!("{:.1} + {:.1}i", self.re, self.im)
        } else {
            format!("{} + {}i", self.re, self.im)
        }
    }

    fn conj(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    fn get_re(self) -> f32 {
        self.re
    }
}

pub trait Lerp<T> {
    fn lerp(u: Self, v: Self, t: T) -> Self;
}

impl Lerp<f32> for f32 {
    fn lerp(u: Self, v: Self, t: f32) -> Self {
        t.mul_add(v - u, u)
    }
}

impl Lerp<f32> for Complex {
    fn lerp(u: Self, v: Self, t: f32) -> Self {
        // 공식: u + (v - u) * t
        // t를 Complex::from_f32(t)로 변환하여 복소수 곱셈 수행
        Complex::from_f32(t).mul_add(v - u, u)
    }
}