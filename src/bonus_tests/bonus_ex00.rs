use crate::utils::{Vector, Matrix, Complex};

pub fn bonus_ex00() {
    let mut u = Vector::from(vec![Complex::new(2., 1.), Complex::new(3., -1.)]);
    let v = Vector::from(vec![Complex::new(5., 2.), Complex::new(7., 4.)]);
    
    u.add(v);
    println!("{}", u); 
    // [7.0 + 3.0i] 
    // [10.0 + 3.0i]

    let mut u = Vector::from(vec![Complex::new(2., 1.), Complex::new(3., -1.)]);
    u.scl(Complex::new(2., 0.));
    println!("{}", u); 
    // [4.0 + 2.0i]
    // [6.0 - 2.0i]

    // ------------------------------------------

    let mut m1 = Matrix::from([
        [Complex::new(1., 1.), Complex::new(2., 0.)],
        [Complex::new(3., -1.), Complex::new(4., 2.)]
    ]);
    let m2 = Matrix::from([
        [Complex::new(7., 2.), Complex::new(4., 4.)],
        [Complex::new(-2., 0.), Complex::new(2., -2.)]
    ]);

    m1.add(m2);
    println!("{}", m1);
}