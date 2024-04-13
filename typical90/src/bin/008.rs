use ac_library::ModInt1000000007 as Mint;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        _: usize,
        S:Chars,
    }

    let atcoder: Vec<char> = "atcoder".chars().collect();
    let char2num: HashMap<char, usize> = atcoder.iter().enumerate().map(|(i, x)| (*x, i)).collect();

    let mut dp = vec![Mint::new(0); atcoder.len() + 1];
    dp[0] = Mint::new(1);

    for s in S.iter() {
        if let Some(&from) = char2num.get(s) {
            let tmp = dp[from];
            dp[from + 1] += tmp;
        }
    }

    println!("{}", dp.last().unwrap());
}
