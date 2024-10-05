use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    // dp[i 匹目見た][ % 2 匹倒した]
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 2]; N + 1];

    // 配る dp
    for (i, &a) in A.iter().enumerate() {
        let tmp = dp[i].clone();
        // 倒す
        dp[i + 1][1] = dp[i + 1][1].max(tmp[0] + a);
        if i != 0 {
            dp[i + 1][0] = dp[i + 1][0].max(tmp[1] + a * 2);
        }

        // 倒さない
        dp[i + 1][1] = dp[i + 1][1].max(tmp[1]);
        dp[i + 1][0] = dp[i + 1][0].max(tmp[0]);
    }
    println!("{}", dp[N].iter().max().unwrap());
}
