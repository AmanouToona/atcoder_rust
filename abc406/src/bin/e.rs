use ac_library::ModInt998244353 as Mint;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {T: usize}

    for _ in 0..T {
        input! {
            (N, K): (usize, usize)
        }

        // bit dp
        // dp[i: 下からの桁数][j: smaller][k: popcount][l: 0-bit count, 1-sum] = l=0 -> 何通りか?  l=1 -> 総和
        let mut dp = vec![vec![vec![vec![Mint::new(0); 2]; K + 1]; 2]; 62];
        dp[61][0][0][0] = Mint::new(1);

        for i in (1..=61).rev() {
            for j in 0..2 {
                for k in 0..=K {
                    let now = dp[i][j][k][0];
                    let now_1 = dp[i][j][k][1];
                    for a in 0..2 {
                        let nxt_i = i - 1;

                        let nxt_k = k + a;
                        if nxt_k > K {
                            continue;
                        }

                        let mut nxt_j = j;
                        if a < (N >> nxt_i & 1) {
                            nxt_j = 1
                        }
                        if a > (N >> nxt_i & 1) && j == 0 {
                            continue;
                        }

                        dp[nxt_i][nxt_j][nxt_k][0] += now;
                        dp[nxt_i][nxt_j][nxt_k][1] += now_1;
                        dp[nxt_i][nxt_j][nxt_k][1] += Mint::new(a) * now * Mint::new(1i64 << nxt_i);
                    }
                }
            }
        }

        let ans = dp[0][0][K][1] + dp[0][1][K][1];
        println!("{ans}");
    }
}
