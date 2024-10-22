use itertools::Itertools;
use proconio::input;
use std::collections::{HashMap, HashSet};
#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W, M): (usize, usize, usize),
        TAX: [(usize, usize, usize); M],
    }

    let mut hs: HashSet<usize> = (0..H).collect();
    let mut ws: HashSet<usize> = (0..W).collect();

    let mut ans = HashMap::new();

    for &(t, a, x) in TAX.iter().rev() {
        let a = a - 1;

        // 行
        if t == 1 {
            if !hs.contains(&a) {
                continue;
            }
            hs.remove(&a);

            if ws.is_empty() {
                continue;
            };
            *ans.entry(x).or_insert(0) += ws.len();
        }

        if t == 2 {
            if !ws.contains(&a) {
                continue;
            }
            ws.remove(&a);

            if hs.is_empty() {
                continue;
            }
            *ans.entry(x).or_insert(0) += hs.len();
        }
    }

    if ws.len() * hs.len() != 0 {
        *ans.entry(0).or_insert(0) += ws.len() * hs.len();
    }

    println!("{}", ans.len());

    for k in ans.keys().sorted() {
        println!("{} {}", k, ans[k]);
    }
}
