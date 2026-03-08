pub trait Operations {
    fn fma(a: Self, b: Self, c: Self) -> Self;
}

impl Operations for f32 {
    fn fma(a: Self, b: Self, c: Self) -> Self {
        a.mul_add(b, c)
    }
}
