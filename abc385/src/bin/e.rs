use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        H: [usize; N],
    }
    // dp[pos][width] = cnt
    let mut dp = vec![vec![0; N]; N];

    for i in 0..N {
        dp[i][0] = 1;
    }

    // 幅
    for w in 1..N {
        // 開始位置
        for u in 0..N {
            let v = u + w;
            if v >= N {
                break;
            }

            dp[v][w] = dp[u][w] + 1;
        }
    }

    let mut ans = 0;

    for w in 0..N {
        for u in 0..N {
            ans = ans.max(dp[u][w]);
        }
    }

    println!("{ans}");
}
