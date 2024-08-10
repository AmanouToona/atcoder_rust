use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {(N, A, T): (usize, usize, usize)}

    if A >= (N + 1) / 2 || T >= (N + 1) / 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
