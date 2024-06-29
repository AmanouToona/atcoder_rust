use ac_library::ModInt1000000007 as Mint;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: Chars,
    }

    // dp [n文字目][最後の文字の状態: A| AA; 0, B | BB: 1,AB : 2, BA: 3]
    let mut dp = vec![vec![Mint::new(0); 4]; N];

    // dp 初期化
    if S[0] == 'A' {
        dp[0][0] += 1;
    } else {
        dp[0][1] += 1;
    }

    for (i, s) in S.iter().skip(1).enumerate() {
        if *s == 'A' {
            // 削除しない
            let t = dp[i][0];
            dp[i + 1][0] += t;
            let t = dp[i][3];
            dp[i + 1][0] += t;

            let t = dp[i][2];
            dp[i + 1][3] += t;

            let t = dp[i][1];
            dp[i + 1][3] += t;

            // 削除する
            if dp[i][2] != Mint::new(0) && i >= 1 {
                let t = dp[i - 1][0];
                dp[i + 1][0] += t;
            }
        } else if *s == 'B' {
            // 削除しない
            let t = dp[i][1];
            dp[i + 1][1] += t;
            let t = dp[i][2];
            dp[i + 1][1] += t;

            let t = dp[i][0];
            dp[i + 1][2] += t;
            let t = dp[i][3];
            dp[i + 1][2] += t;

            // 削除する
            if dp[i][3] != Mint::new(0) && i >= 1 {
                let t = dp[i - 1][1];
                dp[i + 1][1] += t;
            }
        }
        // println!("{:?}", dp[i + 1]);
    }

    let ans: Mint = dp[N - 1].iter().sum();

    println!("{ans}");
}
