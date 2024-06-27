use ac_library::ModInt1000000007 as Mint;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, L): (usize, usize),
    }

    let mut dp = vec![Mint::new(0); N + 1];
    dp[0] += 1;

    'outer: for i in 0..=N {
        for &d in [1, L].iter() {
            let j = i + d;
            if j > N {
                continue 'outer;
            }

            let t = dp[i];
            dp[j] += t;
        }
    }

    println!("{}", dp[N]);
}
