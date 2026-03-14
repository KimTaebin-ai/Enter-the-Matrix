use crate::utils::{Matrix, Complex};

pub fn bonus_ex11() {
    let u = Matrix::from([
        [Complex::new(1., 1.), Complex::new(-1., -1.)],
        [Complex::new(-1., -1.), Complex::new(1., 1.)],
    ]);
    println!("{}", u.determinant());
    // 0.0 + 0.0i

    let u = Matrix::from([
        [Complex::new(2., 0.), Complex::new(0., 0.), Complex::new(0., 0.)],
        [Complex::new(0., 0.), Complex::new(0., 2.), Complex::new(0., 0.)],
        [Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(2., 0.)],
    ]);
    println!("{}", u.determinant());
    // 0.0 + 8.0i

    let u = Matrix::from([
        [Complex::new(8., 0.), Complex::new(5., 0.), Complex::new(-2., 0.)],
        [Complex::new(4., 0.), Complex::new(7., 0.), Complex::new(20., 0.)],
        [Complex::new(7., 0.), Complex::new(6., 0.), Complex::new(1., 0.)],
    ]);
    println!("{}", u.determinant());
    // -174.0 + 0.0i

    let u = Matrix::from([
        [Complex::new(8., 0.), Complex::new(5., 0.), Complex::new(-2., 0.), Complex::new(4., 0.)],
        [Complex::new(4., 0.), Complex::new(2.5, 0.), Complex::new(20., 0.), Complex::new(4., 0.)],
        [Complex::new(8., 0.), Complex::new(5., 0.), Complex::new(1., 0.), Complex::new(4., 0.)],
        [Complex::new(28., 0.), Complex::new(-4., 0.), Complex::new(17., 0.), Complex::new(1., 0.)],
    ]);
    println!("{}", u.determinant());
    // 1032.0 + 0.0i
    
}