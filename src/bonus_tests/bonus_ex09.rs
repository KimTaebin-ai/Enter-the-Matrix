use crate::utils::{Matrix, Complex};

pub fn bonus_ex09() {
    let u = Matrix::from([
        [Complex::new(1., 1.), Complex::new(2., 2.)], 
        [Complex::new(3., 3.), Complex::new(4., 4.)]
    ]);
    println!("{}", u.transpose());
    // [1.0 + 1.0i, 3.0 + 3.0i]
    // [2.0 + 2.0i, 4.0 + 4.0i]

    let u = Matrix::from([
        [Complex::new(1., 0.), Complex::new(2., 1.)], 
        [Complex::new(3., -1.), Complex::new(4., 0.)], 
        [Complex::new(5., 2.), Complex::new(6., -2.)]
    ]);
    println!("{}", u.transpose());
    // [1.0 + 0.0i, 3.0 + -1.0i, 5.0 + 2.0i]
    // [2.0 + 1.0i, 4.0 + 0.0i, 6.0 + -2.0i]
}