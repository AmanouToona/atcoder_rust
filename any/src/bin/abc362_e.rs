use ac_library::ModInt998244353 as Mint;
use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [i32; N],
    }

    // dp[後端の数字][階差][長さ] = 個数
    let mut dp: HashMap<(i32, i32, usize), Mint> = HashMap::new();

    for &a in A.iter() {
        let mut n_dp: HashMap<(i32, i32, usize), Mint> = HashMap::new();
        *n_dp.entry((a, i32::MAX, 1)).or_insert(Mint::new(0)) += 1;

        for (key, v) in dp.into_iter() {
            *n_dp.entry(key).or_insert(Mint::new(0)) += v;

            let r = a - key.0;

            if r == key.1 || key.1 == i32::MAX {
                *n_dp.entry((a, r, key.2 + 1)).or_insert(Mint::new(0)) += v;
            }
        }

        dp = n_dp;
    }

    let mut ans = vec![Mint::new(0); N + 1];

    for ((_, _, len), &cnt) in dp.iter() {
        ans[*len] += cnt;
    }

    let ans: String = ans.iter().skip(1).join(" ");
    println!("{ans}");
}
