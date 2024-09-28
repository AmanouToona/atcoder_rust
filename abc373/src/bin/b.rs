use proconio::input;
use proconio::marker::Chars;
use std::collections::BTreeMap;
#[allow(non_snake_case)]
fn main() {
    input! {S: Chars}

    let mut pos = BTreeMap::new();

    for (i, s) in S.iter().enumerate() {
        pos.insert(*s, i);
    }

    let mut ans = 0;

    let alpha: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    for i in 0..alpha.len() - 1 {
        let from = alpha[i];
        let to = alpha[i + 1];

        ans += pos.get(&from).unwrap().abs_diff(*pos.get(&to).unwrap());
    }

    println!("{ans}");
}
