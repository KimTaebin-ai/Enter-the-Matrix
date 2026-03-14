use crate::utils::{Matrix, Complex};

pub fn bonus_ex08() {
    let u = Matrix::from([
        [Complex::new(1., 0.), Complex::new(0., 0.)],
        [Complex::new(0., 0.), Complex::new(1., 0.)],
    ]);
    println!("{}", u.trace());
    // 2.0 + 0.0i

    let u = Matrix::from([
        [Complex::new(2., 1.), Complex::new(-5., 0.), Complex::new(0., 0.)],
        [Complex::new(4., 0.), Complex::new(3., 2.), Complex::new(7., 0.)],
        [Complex::new(-2., 0.), Complex::new(3., 0.), Complex::new(4., -3.)],
    ]);
    println!("{}", u.trace());
    // 9.0 + 0.0i

    let u = Matrix::from([
        [Complex::new(-2., -1.), Complex::new(-8., 0.), Complex::new(4., 0.)],
        [Complex::new(1., 0.), Complex::new(-23., 5.), Complex::new(4., 0.)],
        [Complex::new(0., 0.), Complex::new(6., 0.), Complex::new(4., 2.)],
    ]);
    println!("{}", u.trace());
    // -21.0 + 6.0i
}