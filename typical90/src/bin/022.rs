use num::Integer;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (A, B, C): (usize, usize, usize),
    }

    let D = A.gcd(&B).gcd(&C);

    let ans = (A / D - 1) + (B / D - 1) + (C / D - 1);
    println!("{}", ans);
}
