use proconio::input;
use std::f64::consts::PI;

#[allow(non_snake_case)]
fn main() {
    input! {
        T: f64,
        (L, X, Y): (f64, f64, f64),
        Q: usize,
    }

    for _ in 0..Q {
        input! {E: f64}
        let E = E % T;

        let uy = -L / 2. * f64::sin(2. * PI * E / T);
        let uz = -L / 2. * f64::cos(2. * PI * E / T) + L / 2.;

        let h = (X.powf(2.) + (uy - Y).powf(2.)).sqrt();
        let v = uz;

        let theta = v.atan2(h) / PI * 180.;

        println!("{}", theta);
    }
}
