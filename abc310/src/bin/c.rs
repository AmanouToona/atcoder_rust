use std::collections::HashSet;

use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }

    let mut set: HashSet<String> = HashSet::new();

    let mut ans = 0;
    for _ in 0..N {
        input! {
            S: Chars,
        }

        let s1: String = S.iter().join("");
        let s2: String = S.iter().rev().join("");

        if !set.contains(&s1) && !set.contains(&s2) {
            ans += 1;
        }

        set.insert(s1);
        set.insert(s2);
    }

    println!("{ans}");
}
