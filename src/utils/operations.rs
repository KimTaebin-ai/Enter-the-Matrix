pub trait Operations: Sized {
    fn fma(a: Self, b: Self, c: Self) -> Self;
    fn abs(self) -> Self;
    fn sqrt(self) -> Self;
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
}

pub trait Lerp<T> {
    fn lerp(u: Self, v: Self, t: T) -> Self;
}

impl Lerp<f32> for f32 {
    fn lerp(u: Self, v: Self, t: f32) -> Self {
        t.mul_add(v - u, u)
    }
}