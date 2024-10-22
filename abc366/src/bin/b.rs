use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: [Chars; N],
    }

    let mut M = 0;
    for s in S.iter() {
        M = M.max(s.len());
    }

    let mut ans = vec![Vec::new(); M];

    for (i, s) in S.iter().rev().enumerate() {
        for m in 0..M {
            if m >= s.len() {
                ans[m].push('*');
            } else {
                ans[m].push(s[m]);
            }
        }
    }

    for a in ans.iter() {
        let a = a.iter().join("");
        let a = a.trim_end_matches('*');

        println!("{a}");
    }
}
