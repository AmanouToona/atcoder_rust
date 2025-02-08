use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        A: [usize; M],
    }

    let A: HashSet<usize> = A.into_iter().collect();

    let mut ans = Vec::new();

    for i in 1..=N {
        if A.contains(&i) {
            continue;
        }
        ans.push(i);
    }
    ans.sort();

    let ans_n = ans.len();
    let ans: String = ans.iter().join(" ");
    println!("{ans_n}");
    println!("{ans}");
}
