use proconio::input;

fn make_dp(target: usize, vac: &Vec<(usize, i64, usize)>, x: usize) -> Vec<i64> {
    let mut dp = vec![-1; x + 1];
    dp[0] = 0;

    for &(v, a, c) in vac.iter() {
        if v != target {
            continue;
        }

        let mut v_dp = dp.clone();

        for uc in 0..=x {
            if v_dp[uc] == -1 {
                continue;
            }

            // 食べない
            v_dp[uc] = v_dp[uc].max(dp[uc]);

            // 食べる
            let vc = uc + c;
            if vc <= x {
                v_dp[vc] = v_dp[vc].max(dp[uc] + a);
            }
        }
        dp = v_dp;
    }

    for i in 0..x {
        dp[i + 1] = dp[i + 1].max(dp[i]);
    }
    dp
}

#[allow(non_snake_case)]
fn main() {
    input! {
        (N, X):(usize, usize),
        VAC: [(usize, i64, usize); N],
    }

    // dp[総摂取カロリー] = 最大ビタミン摂取量
    let dp1 = make_dp(1, &VAC, X);
    let dp2 = make_dp(2, &VAC, X);
    let dp3 = make_dp(3, &VAC, X);

    let mut i1 = 0;
    let mut i2 = 0;
    let mut i3 = 0;

    for _ in 0..X {
        if dp1[i1] <= dp2[i2] && dp1[i1] <= dp3[i3] {
            i1 += 1;
        } else if dp2[i2] <= dp1[i1] && dp2[i2] <= dp3[i3] {
            i2 += 1;
        } else {
            i3 += 1;
        }
    }

    let ans = dp1[i1].min(dp2[i2]).min(dp3[i3]);
    println!("{ans}");
}
