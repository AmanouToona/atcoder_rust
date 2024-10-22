use proconio::input;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {N: usize, K: usize, A: [usize; N]};

    let a_set: HashSet<usize> = A.iter().filter(|x| **x <= K).copied().collect();
    let unique_sum: usize = a_set.iter().sum();

    let total_sum = (1 + K) * K / 2;
    let ans = total_sum - unique_sum;

    println!("{}", ans);
}
