use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: Chars,
    }

    // 出した手は 0,1, 2 とする
    let mut aoki = Vec::new();
    for &s in S.iter() {
        if s == 'R' {
            aoki.push(0);
        } else if s == 'P' {
            aoki.push(1);
        } else if s == 'S' {
            aoki.push(2);
        } else {
            println!("wrong input");
            return;
        }
    }

    // dp[i回目][出した手] = 最大の勝ち数
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 3]; N + 1];

    for (i, &s) in aoki.iter().enumerate() {
        for pre in 0..3 {
            for nxt in 0..3 {
                if nxt == pre {
                    continue;
                }
                if nxt == s {
                    dp[i + 1][nxt] = dp[i + 1][nxt].max(dp[i][pre]);
                } else if nxt == (s + 1) % 3 {
                    dp[i + 1][nxt] = dp[i + 1][nxt].max(dp[i][pre] + 1);
                }
            }
        }
    }

    let ans = dp[N].iter().max().unwrap();
    println!("{ans}");
}
