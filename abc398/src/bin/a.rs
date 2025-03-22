use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize
    }

    let mut ans = vec!["-"; N];

    if N % 2 == 0 {
        ans[N / 2] = "=";
        ans[N / 2 - 1] = "=";
    } else {
        ans[N / 2] = "=";
    }

    let ans: String = ans.iter().join("");

    println!("{ans}");
}
