use ac_library::ModInt998244353 as Mint;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {T: usize}

    for _ in 0..T {
        input! {
            (N, K): (usize, usize)
        }

        // dp[桁][popcount][is smaller]
        let mut dp = vec![vec![vec![Mint::new(0); 2]; 70]; 70];
        for bit in 0..=60 {
            dp[bit][0][1] = Mint::new(1);
        }

        // N を桁の小さい方から走査する
        for digit in 0..=10 {
            // dp を更新する
            for bit in 0..=digit {
                for cnt in (1..=60).rev() {
                    let pre = dp[bit].clone();
                    if (N >> digit) & 1 == 1 {
                        dp[bit][cnt][0] += pre[cnt - 1][0];
                        dp[bit][cnt][1] += pre[cnt - 1][1] + pre[cnt][0];
                    } else {
                        dp[bit][cnt][1] += pre[cnt - 1][1];
                    }
                }
            }
        }

        let mut ans = Mint::new(0);
        for bit in 0..=60 {
            for cnt in 0..=60 {
                ans += Mint::new(1i64 >> cnt) * dp[bit][cnt][1];
            }
        }

        println!("{ans}");
    }
}
