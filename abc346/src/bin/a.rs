use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut B = Vec::new();

    for (&a1, &a2) in A.iter().zip(A.iter().skip(1)) {
        B.push(a1 * a2);
    }

    let ans = B.iter().map(|x| x.to_string()).join(" ");
    println!("{}", ans);
}
