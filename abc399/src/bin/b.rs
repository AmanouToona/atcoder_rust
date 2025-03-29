use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: [usize; N],
    }

    let mut p = P.clone();
    p.sort_by(|x, y| y.cmp(&x));

    let mut cnt = HashMap::new();

    for (i, j) in p.iter().enumerate() {
        if cnt.keys().contains(j) {
            continue;
        }

        cnt.insert(*j, i + 1);
    }

    for i in P.iter() {
        println!("{}", cnt.get(i).unwrap());
    }
}
