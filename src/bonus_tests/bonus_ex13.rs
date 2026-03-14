use crate::utils::{Matrix, Complex};

pub fn bonus_ex13() {
    // 1. Full Rank (3x3 단위 행렬)
    let u = Matrix::from([
        [Complex::new(1., 0.), Complex::new(0., 0.), Complex::new(0., 0.)],
        [Complex::new(0., 0.), Complex::new(1., 0.), Complex::new(0., 0.)],
        [Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(1., 0.)],
    ]);
    println!("{}", u.rank());
    // 3

    let u = Matrix::from([
        [Complex::new(1., 0.), Complex::new(2., 0.), Complex::new(0., 0.), Complex::new(0., 0.)],
        [Complex::new(2., 0.), Complex::new(4., 0.), Complex::new(0., 0.), Complex::new(0., 0.)],
        [Complex::new(-1., 0.), Complex::new(2., 0.), Complex::new(1., 0.), Complex::new(1., 0.)],
    ]);
    println!("{}", u.rank());
    // 2

    let u = Matrix::from([
        [Complex::new(8., 0.), Complex::new(5., 0.), Complex::new(-2., 0.)],
        [Complex::new(4., 0.), Complex::new(7., 0.), Complex::new(20., 0.)],
        [Complex::new(7., 0.), Complex::new(6., 0.), Complex::new(1., 0.)],
        [Complex::new(21., 0.), Complex::new(18., 0.), Complex::new(7., 0.)],
    ]);
    println!("{}", u.rank());
    // 3
    
}