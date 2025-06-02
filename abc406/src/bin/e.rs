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

        // 上の桁から 桁 dp を実行する
        // 配る dp
        for i in (1..=61).rev() {
            for j in 0..2 {
                for k in 0..=K {
                    let now0 = dp[i][j][k][0];
                    if now0 == Mint::new(0) {
                        continue;
                    }

                    let nxt_i = i - 1;

                    // a で遷移する
                    for a in 0..=1 {
                        let mut nxt_j = j;
                        if a > (N >> nxt_i & 1) && j == 0 {
                            continue;
                        }

                        if a < (N >> nxt_i & 1) {
                            nxt_j = 1
                        };

                        let nxt_k = k + a;
                        if nxt_k > K {
                            continue;
                        }

                        dp[nxt_i][nxt_j][nxt_k][0] += now0;
                    }
                }
            }
        }
        println!("{}", dp[0][1][K][0]);
        println!("{}", dp[0][0][K][0]);
    }
}
