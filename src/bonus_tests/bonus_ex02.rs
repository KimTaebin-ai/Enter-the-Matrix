use crate::utils::{Vector, Matrix, Complex, lerp};

pub fn bonus_ex02() {
    let c1 = Complex::new(0., 0.);
    let c2 = Complex::new(10., 10.);
    println!("{}", lerp(c1, c2, 0.5));
    // 5.0 + 5.0i

    let v1 = Vector::from(vec![
        Complex::new(2., 1.), 
        Complex::new(1., 0.)
    ]);
    let v2 = Vector::from(vec![
        Complex::new(4., 3.), 
        Complex::new(2., 2.)
    ]);
    println!("{}", lerp(v1, v2, 0.3));
    // 계산: (4-2)*0.3 + 2 = 2.6, (3-1)*0.3 + 1 = 1.6
    // [2.6 + 1.6i]
    // [1.3 + 0.6i]

    let m1 = Matrix::from([
        [Complex::new(2., 0.), Complex::new(1., 0.)],
        [Complex::new(3., 0.), Complex::new(4., 0.)]
    ]);
    let m2 = Matrix::from([
        [Complex::new(20., 10.), Complex::new(10., 5.)],
        [Complex::new(30., 15.), Complex::new(40., 20.)]
    ]);
    println!("{}", lerp(m1, m2, 0.5));
    // [11.0 + 5.0i, 5.5 + 2.5i]
    // [16.5 + 7.5i, 22.0 + 10.0i]
}