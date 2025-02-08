use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: [usize; N],
        Q: [usize; N],
    }

    let mut ans = vec![0; N];

    for (i, &q) in Q.iter().enumerate() {
        // ゼッケン q を身に着けている
        let q = q - 1;

        // 人 p を見つめている
        let p = P[i] - 1;

        // 人 p が身に着けているゼッケンは
        ans[q] = Q[p];
    }

    let ans: String = ans.iter().join(" ");
    println!("{ans}");
}
