use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
            (N, K, X): (usize, usize, usize),
    mut         A: [usize; N],
        }

    A.insert(K, X);
    let ans = A.iter().join(" ");

    println!("{}", ans);
}
