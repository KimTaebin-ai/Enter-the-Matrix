pub trait Operations {
    fn fma(a: Self, b: Self, c: Self) -> Self;
}

impl Operations for f32 {
    fn fma(a: Self, b: Self, c: Self) -> Self {
        a.mul_add(b, c)
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