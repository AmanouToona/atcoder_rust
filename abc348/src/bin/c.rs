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
        // *min.entry(a).or_insert(c) = *min.entry(a).or_insert(c).min(c);
        min.entry(c)
            .and_modify(|e: &mut usize| *e = (*e).min(a))
            .or_insert(a);
    }

    let mut ans = 0;
    let mut ans_v = 0;

    for (c, v) in min {
        if v <= ans_v {
            continue;
        }
        // ans = c;
        ans_v = v;
    }

    println!("{}", ans_v);
}
