use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K) : (usize, usize),
        S: Chars,
    }

    let mut dp = vec![vec![N; 26]; N];
    for (i, s) in S.iter().enumerate() {
        let s = *s as usize - 'a' as usize;
        dp[i][s] = i;
    }

    for i in (1..N).rev() {
        for s in 0..26 {
            dp[i - 1][s] = dp[i - 1][s].min(dp[i][s]);
        }
    }

    let mut ans = Vec::new();

    let mut u = 0;
    while ans.len() < K {
        for s in 'a'..='z' {
            let v = dp[u][s as usize - 'a' as usize];
            if v == N {
                continue;
            }

            if ans.len() + 1 + (N - 1 - v) >= K {
                ans.push(s);
                u = v + 1;
                break;
            }
        }
    }

    let ans: String = ans.iter().collect();
    println!("{}", ans);
}
