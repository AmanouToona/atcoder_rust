use ac_library::modint::ModInt1000000007 as Mint;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {K: usize}

    if K % 9 != 0 {
        println!("0");
        return;
    }

    // dp[各桁の和]
    let mut dp = vec![Mint::new(0); K + 1];
    dp[0] += 1;

    'outer: for u in 0..K {
        for i in 1..=9 {
            let v = u + i;
            if v > K {
                continue 'outer;
            }

            let tmp = dp[u];
            dp[v] += tmp;
        }
    }

    println!("{}", dp[K]);
}
