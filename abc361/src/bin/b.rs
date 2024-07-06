use proconio::input;
use std::mem;
#[allow(non_snake_case)]
fn main() {
    input! {
        (mut a, mut b, mut c, mut d,mut e,mut  f): (usize, usize, usize, usize, usize, usize),
        (mut g, mut h, mut i, mut j,mut k,mut  l): (usize, usize, usize, usize, usize, usize)
    };

    if a > d {
        mem::swap(&mut a, &mut b);
    }
    if b > e {
        mem::swap(&mut b, &mut e);
    }
    if c > f {
        mem::swap(&mut c, &mut f);
    }

    if g > j {
        mem::swap(&mut g, &mut j);
    }
    if h > k {
        mem::swap(&mut h, &mut k);
    }
    if i > l {
        mem::swap(&mut i, &mut l);
    }

    if j <= a || g >= d {
        println!("No");
        return;
    }

    if k <= b || h >= e {
        println!("No");
        return;
    }
    if l <= c || i >= f {
        println!("No");
        return;
    }
    println!("Yes")
}
