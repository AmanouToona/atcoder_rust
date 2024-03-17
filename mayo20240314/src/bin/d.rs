// https://atcoder.jp/contests/abc318/tasks/abc318_e
use proconio::input;
use std::collections::HashMap;
#[allow(non_snake_case)]

fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut pos = HashMap::new();
    for (i, a) in A.iter().enumerate() {
        pos.entry(a).or_insert(vec![]).push(i);
    }

    let mut ans = 0;
    for val in pos.values_mut() {
        if val.len() < 2 {
            continue;
        }
        val.sort();

        let mut sum_i = 0;
        for (i, v) in val[1..].iter().enumerate() {
            sum_i += val[i];
            ans += (v - 1) * (i + 1) - sum_i;
        }

        if val.len() >= 3 {
            let n = val.len();
            ans -= (n * (n - 1) * (n - 2)) / 6;
        }
    }

    println!("{}", ans);
}
