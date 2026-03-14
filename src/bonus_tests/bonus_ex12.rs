use crate::utils::{Matrix, Complex};

pub fn bonus_ex12() {
    let u = Matrix::from([
        [Complex::new(1., 0.), Complex::new(0., 0.), Complex::new(0., 0.)],
        [Complex::new(0., 0.), Complex::new(1., 0.), Complex::new(0., 0.)],
        [Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(1., 0.)],
    ]);
    println!("{}", u.inverse());
    // [1.0 + 0.0i, 0.0 + 0.0i, 0.0 + 0.0i]
    // [0.0 + 0.0i, 1.0 + 0.0i, 0.0 + 0.0i]
    // [0.0 + 0.0i, 0.0 + 0.0i, 1.0 + 0.0i]

    let u = Matrix::from([
        [Complex::new(0., 2.), Complex::new(0., 0.), Complex::new(0., 0.)],
        [Complex::new(0., 0.), Complex::new(0., 2.), Complex::new(0., 0.)],
        [Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(0., 2.)],
    ]);
    println!("{}", u.inverse());
    // [0.0 + 0.5i, 0.0 + 0.0i, 0.0 + 0.0i]
    // [0.0 + 0.0i, 0.0 + 0.5i, 0.0 + 0.0i]
    // [0.0 + 0.0i, 0.0 + 0.0i, 0.0 + 0.5i]

    let u = Matrix::from([
        [Complex::new(8., 0.), Complex::new(5., 0.), Complex::new(-2., 0.)],
        [Complex::new(4., 0.), Complex::new(7., 0.), Complex::new(20., 0.)],
        [Complex::new(7., 0.), Complex::new(6., 0.), Complex::new(1., 0.)],
    ]);
    println!("{}", u.inverse());
    // [0.64942527 + 0.0i, 0.09770115 + 0.0i, -0.6551724 + 0.0i]
    // [-0.7816092 + 0.0i, -0.12643678 + 0.0i, 0.9655172 + 0.0i]
    // [0.14367816 + 0.0i, 0.07471265 + 0.0i, -0.20689656 + 0.0i]
}