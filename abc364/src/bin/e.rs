use std::usize;

use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, X, Y): (usize, usize, usize),
    mut     AB: [(usize, usize); N],
    }

    // dp [i個目までみた][合計の甘さ sa][食べた個数 m] = 合計のしょっぱさの最小値 sb
    let mut dp = vec![vec![vec![usize::MAX; N + 1]; X + 2]; N + 1];
    dp[0][0][0] = 0;

    // 配る dp
    for (i, &(a, b)) in AB.iter().enumerate() {
        for sa in 0..X {
            for m in 0..=i {
                // 食べない
                dp[i + 1][sa][m] = dp[i + 1][sa][m].min(dp[i][sa][m]);

                // 食べる
                if dp[i][sa][m] > Y {
                    continue;
                }

                let nxt_a = (sa + a).min(X + 1);
                dp[i + 1][nxt_a][m + 1] = dp[i + 1][nxt_a][m + 1].min(dp[i][sa][m] + b);
            }
        }
    }

    // 答え
    let mut ans = 0;
    for i in 0..=N {
        for a in 0..=X {
            for m in 0..=i {
                if dp[i][a][m] <= Y {
                    ans = ans.max(m + 1);
                }
            }
        }
    }
    ans = ans.min(N);
    println!("{ans}")
}
