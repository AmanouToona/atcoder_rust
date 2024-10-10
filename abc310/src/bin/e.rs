use proconio::input;
use proconio::marker::Chars;
use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: Chars,
    }

    let S: Vec<usize> = S
        .into_iter()
        .map(|x| if x == '0' { 0 } else { 1 })
        .collect();

    let mut nand = vec![S[0]];
    for &s in S.iter().skip(1) {
        nand.push(1 - (nand.last().unwrap() & s));
    }

    let mut cumsum = vec![0];
    for &i in nand.iter() {
        cumsum.push(cumsum.last().unwrap() + i);
    }

    let mut pos_0 = BTreeSet::new();
    for (i, &s) in S.iter().enumerate() {
        if s == 0 {
            pos_0.insert(i);
        }
    }
    pos_0.insert(N);

    let mut ans = 0;

    for (i, (&s, &n)) in S.iter().zip(nand.iter()).enumerate() {
        if s == n {
            ans += cumsum.last().unwrap() - cumsum[i];
        } else {
            let &next_0 = pos_0.range((Excluded(i), Unbounded)).next().unwrap();

            ans += cumsum.last().unwrap() - cumsum[next_0];
            ans += next_0 - i - (cumsum[next_0] - cumsum[i]);
        }
    }

    println!("{ans}");
}
