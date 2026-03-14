use crate::utils::{Vector, Complex};
use crate::utils::vector::cross_product;

pub fn bonus_ex06() {
    let u = Vector::from(vec![
        Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(1., 0.)
    ]);
    let v = Vector::from(vec![
        Complex::new(1., 0.), Complex::new(0., 0.), Complex::new(0., 0.)
    ]);
    println!("{}", cross_product(&u, &v));
    // [0.0 + 0.0i]
    // [1.0 + 0.0i]
    // [0.0 + 0.0i]

    let u = Vector::from(vec![
        Complex::new(0., 1.), Complex::new(0., 0.), Complex::new(0., 0.)
    ]);
    let v = Vector::from(vec![
        Complex::new(0., 0.), Complex::new(0., 1.), Complex::new(0., 0.)
    ]);
    println!("{}", cross_product(&u, &v));
    // [0.0 + 0.0i]
    // [0.0 + 0.0i]
    // [-1.0 + 0.0i]

    let u = Vector::from(vec![
        Complex::new(1., 0.), Complex::new(2., 0.), Complex::new(3., 0.)
    ]);
    let v = Vector::from(vec![
        Complex::new(4., 0.), Complex::new(5., 0.), Complex::new(6., 0.)
    ]);
    println!("{}", cross_product(&u, &v));
    // [-3.0 + 0.0i]
    // [6.0 + 0.0i]
    // [-3.0 + 0.0i]

    let u = Vector::from(vec![
        Complex::new(1., 0.), Complex::new(2., 3.), Complex::new(3., 1.)
    ]);
    let v = Vector::from(vec![
        Complex::new(4., 0.), Complex::new(5., 1.), Complex::new(6., 4.)
    ]);
    println!("{}", cross_product(&u, &v));
    // [-14.0 + 18.0i]
    // [6.0 + 0.0i]
    // [-3.0 + -11.0i]

}