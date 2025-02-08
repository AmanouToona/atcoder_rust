use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (A, B, C): (usize, usize, usize)
    }

    if A + B == C || A + C == B || B + C == A {
        println!("Yes");
    } else if A == B && B == C {
        println!("Yes");
    } else {
        println!("No")
    }
}
