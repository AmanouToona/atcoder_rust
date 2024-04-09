use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
    N: usize,
    H: [usize; N],
    }

    let mut dp = vec![usize::MAX; N];

    dp[0] = 0;

    for i in 0..(N - 1) {
        for d in 1..=2 {
            let nxt = i + d;
            if nxt >= N {
                continue;
            }
            dp[nxt] = dp[nxt].min(H[i].abs_diff(H[nxt]) + dp[i]);
        }
    }

    println!("{}", dp[N - 1]);
}
