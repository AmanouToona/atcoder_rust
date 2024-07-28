use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, X, Y): (usize, usize, usize),
        AB: [(usize, usize); N],
    }

    // dp [i個目までみた][合計の甘さ sa][食べた個数 m] = 合計のしょっぱさの最小値 sb
    let mut dp = vec![vec![vec![usize::max; N + 1]; X + 1]; N + 1];
    dp[0][0][0] = 0;

    // 配る dp
    for i in 0..N {
        for sa in 0..=X {
            for m in 0..=i {
                for &(a, b) in AB.iter() {
                    let nxt_a = (sa + a).min(X);
                }
            }
        }
    }
}
