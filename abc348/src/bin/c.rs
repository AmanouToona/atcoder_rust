use proconio::input;
use std::collections::HashMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        AC: [(usize, usize); N],
    }

    let mut min = HashMap::new();

    for (a, c) in AC.into_iter() {
        min.entry(c)
            .and_modify(|e: &mut usize| *e = (*e).min(a))
            .or_insert(a);
    }

    let ans = min.values().max().unwrap();
    println!("{}", ans);
}
