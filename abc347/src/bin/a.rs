use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
        A : [usize; N],
    }

    let ans: String = A
        .iter()
        .filter(|x| *x % K == 0)
        .map(|x| *x / K)
        .clone()
        .map(|x| x.to_string())
        .join(" ");
    println!("{}", ans);
}
