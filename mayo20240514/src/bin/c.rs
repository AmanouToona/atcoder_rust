//https://atcoder.jp/contests/abc107/tasks/abc107_b
use itertools::iproduct;
use proconio::input;
use proconio::marker::Chars;
use std::collections::BTreeSet;

#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W): (usize, usize),
        A: [Chars; H],
    }

    let mut hs: BTreeSet<usize> = BTreeSet::new();
    let mut ws: BTreeSet<usize> = BTreeSet::new();

    for (h, w) in iproduct!(0..H, 0..W) {
        if A[h][w] == '#' {
            hs.insert(h);
            ws.insert(w);
        }
    }

    let mut ans: Vec<Vec<char>> = vec![vec!['a'; ws.len()]; hs.len()];

    for (i, &h) in hs.iter().enumerate() {
        for (j, &w) in ws.iter().enumerate() {
            ans[i][j] = A[h][w];
        }
    }

    for a in ans.iter() {
        let a: String = a.iter().collect();
        println!("{}", a);
    }
}
