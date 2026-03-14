use crate::utils::{Vector, Complex};

pub fn bonus_ex04() {
    let u = Vector::from(vec![
        Complex::new(0., 0.), 
        Complex::new(0., 0.), 
        Complex::new(0., 0.)
    ]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 0.0 + 0.0i, 0.0 + 0.0i, 0.0 + 0.0i
    let u = Vector::from(vec![Complex::new(3., 4.)]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 5.0 + 0.0i, 3.0 + 4.0i, 5.0 + 0.0i
    let u = Vector::from(vec![
        Complex::new(1., 1.), 
        Complex::new(1., -1.)
    ]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 2.8 + 0.0i, 0.0 + 0.0i, 1.4 + 0.0i
}