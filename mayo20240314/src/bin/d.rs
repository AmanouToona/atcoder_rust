use alga::general::Ring;
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

        for i in 0..val.len() - 1 {
            let left = i + 1;
            let right = val.len() - (i + 1);
            ans += (val[i + 1] - val[i] - 1) * left * right;
        }
    }

    println!("{}", ans);
}
