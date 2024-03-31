use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, A, B): (usize, usize, usize),
        D: [usize; N],
    }

    let d: HashSet<usize> = D.into_iter().map(|x| x % (A + B)).collect();
    let mut d: Vec<usize> = d.into_iter().collect();
    d.sort();

    if d.last().unwrap() - d.first().unwrap() < A
        || d.iter().tuple_windows().any(|(d1, d2)| d2 - d1 > B)
    {
        println!("Yes");
        return;
    }

    println!("No")
}
