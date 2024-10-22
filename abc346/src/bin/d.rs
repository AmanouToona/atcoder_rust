use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! { N: usize, S: Chars,C: [usize; N]};

    // dp  n文字目, 一致文字あり, 前の字
    let mut dp = vec![vec![vec![usize::MAX; 2]; 2]; N];

    if S[0] == '0' {
        dp[0][0][0] = 0;
        dp[0][0][1] = C[0];
    } else {
        dp[0][0][0] = C[0];
        dp[0][0][1] = 0;
    }

    for n in 0..N - 1 {
        for same in 0..2 {
            for u in 0..2 {
                if dp[n][same][u] == usize::MAX {
                    continue;
                }

                // println!("{} {} {} {}", n, same, u, dp[n][same][u]);

                if same == 1 {
                    let nxt: usize = if u == 1 { 0 } else { 1 };
                    let nxt_c = if nxt == 1 { '1' } else { '0' };

                    let cost = if S[n + 1] != nxt_c { C[n + 1] } else { 0 };

                    dp[n + 1][1][nxt] = dp[n + 1][1][nxt].min(dp[n][same][u] + cost);
                    continue;
                }

                // 一致なしへの遷移
                let nxt: usize = if u == 1 { 0 } else { 1 };
                let nxt_c = if nxt == 1 { '1' } else { '0' };

                let cost = if S[n + 1] != nxt_c { C[n + 1] } else { 0 };
                dp[n + 1][0][nxt] = dp[n + 1][0][nxt].min(dp[n][same][u] + cost);

                // 一致ありへの遷移
                let nxt: usize = u;
                let nxt_c = if nxt == 1 { '1' } else { '0' };

                let cost = if S[n + 1] != nxt_c { C[n + 1] } else { 0 };
                dp[n + 1][1][nxt] = dp[n + 1][1][nxt].min(dp[n][same][u] + cost);
            }
        }
    }

    let ans = dp[N - 1][1][0].min(dp[N - 1][1][1]);
    println!("{}", ans);
}
