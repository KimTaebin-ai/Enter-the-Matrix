use crate::utils::{Matrix, Complex};

pub fn bonus_ex10() {
    let u = Matrix::from([
        [Complex::new(1., 0.), Complex::new(0., 0.), Complex::new(0., 0.)],
        [Complex::new(0., 0.), Complex::new(1., 0.), Complex::new(0., 0.)],
        [Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(1., 0.)],
    ]);
    println!("{}", u.row_echelon());
    // [1.0 + 0.0i, 0.0 + 0.0i, 0.0 + 0.0i]
    // [0.0 + 0.0i, 1.0 + 0.0i, 0.0 + 0.0i]
    // [0.0 + 0.0i, 0.0 + 0.0i, 1.0 + 0.0i]

    let u = Matrix::from([
        [Complex::new(1., 1.), Complex::new(2., 0.)],
        [Complex::new(3., 0.), Complex::new(4., -2.)],
    ]);
    println!("{}", u.row_echelon());
    // [1.0 + 0.0i, 0.0 + 0.0i]
    // [0.0 + 0.0i, 1.0 + 0.0i]

    let u = Matrix::from([
        [Complex::new(1., 2.), Complex::new(2., 4.)],
        [Complex::new(2., 4.), Complex::new(4., 8.)],
    ]);
    println!("{}", u.row_echelon());
    // [1.0 + 0.0i, 2.0 + 0.0i]
    // [0.0 + 0.0i, 0.0 + 0.0i]

    let u = Matrix::from([
        [Complex::new(8., 0.), Complex::new(5., 0.), Complex::new(-2., 0.), Complex::new(4., 0.), Complex::new(28., 0.)],
        [Complex::new(4., 0.), Complex::new(2.5, 0.), Complex::new(20., 0.), Complex::new(4., 0.), Complex::new(-4., 0.)],
        [Complex::new(8., 0.), Complex::new(5., 0.), Complex::new(1., 0.), Complex::new(4., 0.), Complex::new(17., 0.)],
    ]);
    println!("{}", u.row_echelon());
    // [1.0 + 0.0i, 0.6 + 0.0i, 0.0 + 0.0i, 0.0 + 0.0i, -12.2 + 0.0i]
    // [0.0 + 0.0i, 0.0 + 0.0i, 1.0 + 0.0i, 0.0 + 0.0i, -3.7 + 0.0i]
    // [0.0 + 0.0i, 0.0 + 0.0i, 0.0 + 0.0i, 1.0 + 0.0i, 29.5 + 0.0i]
    
}