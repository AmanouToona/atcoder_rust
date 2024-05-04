use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        DCS: [(usize, usize, usize); N],
    }

    // dead line の早い順にソート
    let mut DCS: Vec<(usize, usize, usize)> = DCS.into_iter().filter(|a| a.1 <= a.0).collect();
    // DCS.sort_by(|a, b| (a.0.saturating_sub(a.1)).cmp(&(b.0.saturating_sub(b.1))));
    DCS.sort_by(|a, b| a.0.cmp(&b.0));

    // dp[n 個目の仕事][d 日目] = 最大値
    let mut dp = vec![vec![0; 5002]; DCS.len() + 1];

    for job in 0..DCS.len() {
        // 更新
        for day in 1..=5000 {
            dp[job][day + 1] = dp[job][day + 1].max(dp[job][day]);
        }

        for day in 0..=5000 {
            // 仕事をする
            let end = day + DCS[job].1;

            if end <= DCS[job].0 {
                dp[job + 1][end] = dp[job + 1][end].max(dp[job][day] + DCS[job].2);
            }

            // 仕事をしない
            dp[job + 1][day] = dp[job + 1][day].max(dp[job][day]);
        }
    }

    let ans = dp[DCS.len()].iter().max().unwrap();

    println!("{}", ans);
}
