use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut pos: HashMap<usize, usize> = HashMap::new();

    let mut B: Vec<i64> = Vec::new();

    for (i, &a) in A.iter().enumerate() {
        if let Some(p) = pos.get(&a) {
            B.push(*p as i64);
        } else {
            B.push(-1);
        }

        *pos.entry(a).or_insert(0) = i + 1;
    }

    let ans: String = B.iter().join(" ");
    println!("{ans}");
}
