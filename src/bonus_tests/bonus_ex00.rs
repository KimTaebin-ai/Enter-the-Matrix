use crate::utils::{Vector, Matrix, Complex};

pub fn bonus_ex00() {
    let mut u = Vector::from(vec![Complex::new(2., 1.), Complex::new(3., -1.)]);
    let v = Vector::from(vec![Complex::new(5., 2.), Complex::new(7., 4.)]);
    
    u.add(v);
    println!("{}", u); 
    // [7.0 + 3.0i] 
    // [10.0 + 3.0i]

    let mut u = Vector::from(vec![Complex::new(2., 2.), Complex::new(3., -1.)]);
    let v = Vector::from(vec![Complex::new(5., 2.), Complex::new(7., 4.)]);
    
    u.sub(v);
    println!("{}", u); 
    //[-3.0 + 0.0i]
    // [-4.0 + -5.0i]

    let mut u = Vector::from(vec![Complex::new(2., 1.), Complex::new(3., -1.)]);
    u.scl(Complex::new(2., 0.));
    println!("{}", u); 
    // [4.0 + 2.0i]
    // [6.0 - 2.0i]

    // ------------------------------------------

    let mut u = Matrix::from([
        [Complex::new(1., 1.), Complex::new(2., 0.)],
        [Complex::new(3., -1.), Complex::new(4., 2.)]
    ]);
    let v = Matrix::from([
        [Complex::new(7., 2.), Complex::new(4., 4.)],
        [Complex::new(-2., 0.), Complex::new(2., -2.)]
    ]);

    u.add(v);
    println!("{}", u);
    // [8.0 + 3.0i, 6.0 + 4.0i]
    // [1.0 + -1.0i, 6.0 + 0.0i]

    let mut u = Matrix::from([
        [Complex::new(1., 1.), Complex::new(2., 0.)],
        [Complex::new(3., -1.), Complex::new(4., 2.)]
    ]);
    let v = Matrix::from([
        [Complex::new(7., 2.), Complex::new(4., 4.)],
        [Complex::new(-2., 0.), Complex::new(2., -2.)]
    ]);

    u.sub(v);
    println!("{}", u);
    // [-6.0 + -1.0i, -2.0 + -4.0i]
    // [5.0 + -1.0i, 2.0 + 4.0i]

    let mut u = Matrix::from([
        [Complex::new(1., 1.), Complex::new(2., 0.)],
        [Complex::new(3., -1.), Complex::new(4., 2.)]
    ]);
    u.scl(Complex::new(2., 0.));
    println!("{}", u);
    // [2.0 + 2.0i, 4.0 + 0.0i]
    // [6.0 + -2.0i, 8.0 + 4.0i]


}