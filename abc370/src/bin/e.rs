use ac_library::ModInt998244353 as Mint;
use proconio::input;

use std::collections::BTreeMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, i64),
        A: [i64; N],
    }

    let mut cusum: Vec<i64> = Vec::from([0]);

    for a in A.iter() {
        cusum.push(*a + cusum.last().unwrap());
    }

    let mut map: BTreeMap<i64, Mint> = BTreeMap::new();
    map.insert(0, Mint::new(1));
    let mut all = Mint::new(1);
    let mut dp = Mint::new(0);

    for (i, &a) in A.iter().enumerate() {
        dp = all - *map.entry(cusum[i + 1] - K).or_insert(Mint::new(0));
        eprintln!("dp: {dp}");
        *map.entry(cusum[i + 1]).or_insert(Mint::new(0)) += dp;

        all += dp;
    }

    println!("{dp}");
}
