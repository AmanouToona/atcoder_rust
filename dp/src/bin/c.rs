use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        ABC: [[usize; 3]; N],
    }

    // dp[何日目か][今今日は何をしたか -: A, 1: B, 2: C] = 楽しさの最大値
    let mut dp = vec![vec![0; 3]; N + 1];

    for i in 0..N {
        for j in 0..3 {
            for k in 0..3 {
                // i 日目は j をしていた i + 1 日目は k をする
                if j == k {
                    continue;
                }
                dp[i + 1][k] = dp[i + 1][k].max(dp[i][j] + ABC[i][k]);
            }
        }
    }

    println!("{}", dp[N].iter().max().unwrap());
}
