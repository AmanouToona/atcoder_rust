use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        AB : [(usize, usize); N],
    }

    // dp[team 1 power][team 2 power][use n people] = min switch num
    let mut dp = vec![vec![vec![usize::MAX; N + 1]; 501]; 501];
    dp[0][0][0] = 0;

    let mut total_power = 0;

    for (i, &(a, b)) in AB.iter().enumerate() {
        total_power += b;

        for p_1 in 0..=500 {
            for p_2 in 0..=500 {
                if p_1 + p_2 > total_power {
                    continue;
                }
                if dp[p_1][p_2][i] == usize::MAX {
                    continue;
                }
                for team in [1, 2, 3] {
                    let p_1_nxt = if team == 1 { p_1 + b } else { p_1 };
                    let p_2_nxt = if team == 2 { p_2 + b } else { p_2 };

                    if p_1_nxt > 500 || p_2_nxt > 500 {
                        continue;
                    }

                    if team == a {
                        dp[p_1_nxt][p_2_nxt][i + 1] =
                            dp[p_1_nxt][p_2_nxt][i + 1].min(dp[p_1][p_2][i]);
                    } else {
                        dp[p_1_nxt][p_2_nxt][i + 1] =
                            dp[p_1_nxt][p_2_nxt][i + 1].min(dp[p_1][p_2][i] + 1);
                    }
                }
            }
        }
    }

    let ans = dp[total_power / 3][total_power / 3][N];

    if ans == usize::MAX || total_power % 3 != 0 {
        println!("-1");
    } else {
        println!("{ans}");
    }
}
