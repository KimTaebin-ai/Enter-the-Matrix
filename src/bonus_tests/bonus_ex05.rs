use crate::utils::{Vector, Complex};
use crate::utils::vector::angle_cos;

pub fn bonus_ex05() {
    let u = Vector::from(vec![Complex::new(1., 1.)]);
    let v = Vector::from(vec![Complex::new(1., 1.)]);
    println!("{}", angle_cos(&u, &v));
    // 0.0 + -1.0i

    let u = Vector::from(vec![Complex::new(1., 0.)]);
    let v = Vector::from(vec![Complex::new(0., 1.)]);
    println!("{}", angle_cos(&u, &v));
    // -1.0 + -0.0i

    let u = Vector::from(vec![Complex::new(1., 1.)]);
    let v = Vector::from(vec![Complex::new(-1., -1.)]);
    println!("{}", angle_cos(&u, &v));
    // 0.0 + 1.0i

    let u = Vector::from(vec![Complex::new(1., 2.), Complex::new(3., 4.)]);
    let v = Vector::from(vec![Complex::new(4., 3.), Complex::new(2., 1.)]);
    println!("{}", angle_cos(&u, &v));
    // 0.3 + -0.7i

}