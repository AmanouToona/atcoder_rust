use itertools::{iproduct, Itertools};
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W): (usize, usize),
        A: [[usize; W]; H],
    }

    let mut sum_h = vec![0; H];
    let mut sum_w = vec![0; W];

    for (h, w) in iproduct!(0..H, 0..W) {
        sum_h[h] += A[h][w];
        sum_w[w] += A[h][w];
    }

    let mut ans = vec![vec![0; W]; H];
    for (h, w) in iproduct!(0..H, 0..W) {
        ans[h][w] += sum_h[h] + sum_w[w] - A[h][w];
    }

    for a in ans.iter() {
        let out: String = a.iter().join(" ");
        println!("{}", out);
    }
}
