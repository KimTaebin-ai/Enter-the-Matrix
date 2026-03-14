use crate::utils::{Vector, Complex};

pub fn bonus_ex03() {
    let u = Vector::from(vec![
        Complex::new(0., 0.), 
        Complex::new(0., 0.)
    ]);
    let v = Vector::from(vec![
        Complex::new(1., 1.), 
        Complex::new(1., 1.)
    ]);
    println!("{}", u.dot(&v));
    // 0.0
  
    let u = Vector::from(vec![
        Complex::new(1., 1.), 
        Complex::new(1., 1.)
    ]);
    let v = Vector::from(vec![
        Complex::new(1., 1.), 
        Complex::new(1., 1.)
    ]);
    println!("{}", u.dot(&v));
    // 4.0

    let u = Vector::from(vec![Complex::new(-1., 6.)]);
    let v = Vector::from(vec![Complex::new(3., 2.)]);
    println!("{}", u.dot(&v));
    // 9.0 + 20.0i
}