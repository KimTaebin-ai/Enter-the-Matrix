use crate::utils::{Matrix, Vector, Complex};

pub fn bonus_ex07() {
    let u = Matrix::from([
        [Complex::new(1., 0.), Complex::new(0., 0.)],
        [Complex::new(0., 0.), Complex::new(1., 0.)],
    ]);
    let v = Vector::from(vec![Complex::new(4., 2.), Complex::new(1., 1.)]);
    println!("{}", u.mul_vec(&v));
    // [4.0 + -2.0i]
    // [1.0 + -1.0i]

    let u = Matrix::from([
        [Complex::new(2., 0.), Complex::new(0., 0.)],
        [Complex::new(0., 0.), Complex::new(0., 2.)],
    ]);
    let v = Vector::from(vec![Complex::new(4., 2.), Complex::new(1., -1.)]);
    println!("{}", u.mul_vec(&v));
    // [8.0 + -4.0i]
    // [-2.0 + 2.0i]

    let u = Matrix::from([
        [Complex::new(2., 1.), Complex::new(-2., 0.)],
        [Complex::new(-2., -1.), Complex::new(2., 0.)],
    ]);
    let v = Vector::from(vec![Complex::new(4., 0.), Complex::new(2., 0.)]);
    println!("{}", u.mul_vec(&v));
    // [4.0 + 4.0i]
    // [-4.0 + -4.0i]

    let u = Matrix::from([
        [Complex::new(1., 0.), Complex::new(0., 0.)],
        [Complex::new(0., 0.), Complex::new(1., 0.)],
    ]);
    let v = Matrix::from([
        [Complex::new(1., 0.), Complex::new(0., 0.)],
        [Complex::new(0., 0.), Complex::new(1., 0.)],
    ]);
    println!("{}", u.mul_mat(&v));
    // [1.0 + 0.0i, 0.0 + 0.0i]
    // [0.0 + 0.0i, 1.0 + 0.0i]

    let u = Matrix::from([
        [Complex::new(1., 1.), Complex::new(0., 0.)],
        [Complex::new(0., 0.), Complex::new(1., -1.)],
    ]);
    let v = Matrix::from([
        [Complex::new(2., 0.), Complex::new(1., 0.)],
        [Complex::new(4., 0.), Complex::new(2., 0.)],
    ]);
    println!("{}", u.mul_mat(&v));
    // [2.0 + 2.0i, 1.0 + 1.0i]
    // [4.0 + -4.0i, 2.0 + -2.0i]

    let u = Matrix::from([
        [Complex::new(3., 1.), Complex::new(-5., 0.)],
        [Complex::new(6., 0.), Complex::new(8., -2.)],
    ]);
    let v = Matrix::from([
        [Complex::new(2., 1.), Complex::new(1., 0.)],
        [Complex::new(4., 0.), Complex::new(2., 0.)],
    ]);
    println!("{}", u.mul_mat(&v));
    // [-13.0 + -1.0i, -7.0 + 1.0i]
    // [44.0 + -14.0i, 22.0 + -4.0i]
}