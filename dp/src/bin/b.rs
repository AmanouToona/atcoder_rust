use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
        H: [usize; N],
    }

    let mut dp = vec![usize::MAX; N];

    dp[0] = 0;

    for i in 0..N - 1 {
        for k in 1..=K {
            let j = i + k;
            if j >= N {
                break;
            }
            dp[j] = dp[j].min(dp[i] + H[j].abs_diff(H[i]));
        }
    }
    println!("{}", dp[N - 1]);
}
