use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N],
    }

    let mut ans = Vec::new();

    for a in A.iter().skip(N - K) {
        ans.push(*a);
    }

    for a in A.iter().take(N - K) {
        ans.push(*a);
    }

    let ans: String = ans.iter().join(" ");
    println!("{ans}");
}
