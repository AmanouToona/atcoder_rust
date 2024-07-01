//https://atcoder.jp/contests/abc083/tasks/abc083_a
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (A, B, C, D): (usize, usize, usize, usize),
    }

    if A + B > C + D {
        println!("Left");
    } else if A + B == C + D {
        println!("Balanced");
    } else {
        println!("Right");
    }
}

