use crate::utils::{Vector, Complex};
use crate::utils::vector::linear_combination;

pub fn bonus_ex01() {
    let e1 = Vector::from(vec![
        Complex::new(1., 0.), 
        Complex::new(0., 0.), 
        Complex::new(0., 0.)
    ]);
    let e2 = Vector::from(vec![
        Complex::new(0., 0.), 
        Complex::new(1., 0.), 
        Complex::new(0., 0.)
    ]);
    let e3 = Vector::from(vec![
        Complex::new(0., 0.), 
        Complex::new(0., 0.), 
        Complex::new(1., 0.)
    ]);
    
    // v1 = [1+1i, 2+0i, 3-1i]
    let v1 = Vector::from(vec![
        Complex::new(1., 1.), 
        Complex::new(2., 0.), 
        Complex::new(3., -1.)
    ]);
    // v2 = [0+2i, 10+0i, 0-5i]
    let v2 = Vector::from(vec![
        Complex::new(0., 2.), 
        Complex::new(10., 0.), 
        Complex::new(0., -5.)
    ]);
    
    println!("{}", linear_combination(
        &vec![e1, e2, e3], 
        &vec![
            Complex::new(10., 2.), 
            Complex::new(-2., 0.), 
            Complex::new(0., 0.5)
        ]
    ));
    // [10.0 + 2.0i]
    // [-2.0 + 0.0i]
    // [0.0 + 0.5i]

    println!("{}", linear_combination(
        &vec![v1, v2], 
        &vec![
            Complex::new(2., 0.), 
            Complex::new(0., 1.)
        ]
    ));
    // [0.0 + 2.0i]
    // [4.0 + 10.0i]
    // [11.0 + -2.0i]
}