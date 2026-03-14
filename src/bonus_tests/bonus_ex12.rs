use crate::utils::{Matrix, Complex};

pub fn bonus_ex12() {
    let u = Matrix::from([
        [Complex::new(1., 0.), Complex::new(0., 0.), Complex::new(0., 0.)],
        [Complex::new(0., 0.), Complex::new(1., 0.), Complex::new(0., 0.)],
        [Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(1., 0.)],
    ]);
    println!("{}", u.inverse());

    let u = Matrix::from([
        [Complex::new(0., 2.), Complex::new(0., 0.), Complex::new(0., 0.)],
        [Complex::new(0., 0.), Complex::new(0., 2.), Complex::new(0., 0.)],
        [Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(0., 2.)],
    ]);
    println!("{}", u.inverse());

    let u = Matrix::from([
        [Complex::new(8., 0.), Complex::new(5., 0.), Complex::new(-2., 0.)],
        [Complex::new(4., 0.), Complex::new(7., 0.), Complex::new(20., 0.)],
        [Complex::new(7., 0.), Complex::new(6., 0.), Complex::new(1., 0.)],
    ]);
    println!("{}", u.inverse());
}