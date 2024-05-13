use num::Integer;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (A, B): (i64, i64),
    }

    let gcd = A.gcd(&B);

    if A / gcd > 10i64.pow(18u32) / B {
        println!("Large");
        return;
    }

    let ans = A.lcm(&B);

    println!("{}", ans);
}
