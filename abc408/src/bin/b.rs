use proconio::input;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let A: HashSet<usize> = A.into_iter().collect();
    let mut A: Vec<usize> = A.into_iter().collect();
    A.sort_by(|x, y| x.cmp(&y));

    println!("{}", A.len());
    let ans: String = A
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{ans}");
}
