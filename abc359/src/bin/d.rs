use ac_library::ModInt998244353 as Mint;
use proconio::input;
use proconio::marker::Chars;

fn is(state: usize, k: usize, n: usize) -> usize {
    if n < k {
        return 0;
    }

    for (i, j) in (0..k).zip((0..k).rev()) {
        if (state >> i) & 1 != (state >> j) & 1 {
            return 0;
        }
    }

    1
}

#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
        S: Chars,
    }

    // dp[N 文字目まで見た][最後の状態][回文であるか no 0; yes 1];
    let mut dp = vec![vec![vec![Mint::new(0); 2]; 1 << K]; N + 1];
    dp[0][0][0] = Mint::new(1);
    let mask = (1 << K) - 1;

    for n in 0..N {
        for state in 0..(1 << K) {
            let mut nxt = state << 1;
            let pre_0 = dp[n][state][0];
            let pre_1 = dp[n][state][1];

            if S[n] == 'A' {
                nxt += 1;
                nxt &= mask;

                dp[n + 1][nxt][is(nxt, K, n + 1)] += pre_0;
                dp[n + 1][nxt][1] += pre_1;
                continue;
            } else if S[n] == 'B' {
                nxt += 0;
                nxt &= mask;
                dp[n + 1][nxt][is(nxt, K, n + 1)] += pre_0;
                dp[n + 1][nxt][1] += pre_1;
                continue;
            }

            let mut nx = nxt;
            nx += 1;
            nx &= mask;
            dp[n + 1][nx][is(nx, K, n + 1)] += pre_0;
            dp[n + 1][nx][1] += pre_1;

            let mut nx = nxt;
            nx &= mask;
            dp[n + 1][nx][is(nx, K, n + 1)] += pre_0;
            dp[n + 1][nx][1] += pre_1;
        }
    }

    let mut ans = Mint::new(0);
    for s in 0..(1 << K) {
        ans += dp[N][s][0];
    }

    println!("{}", ans);
}
