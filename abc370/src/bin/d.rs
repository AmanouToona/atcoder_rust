use proconio::input;
use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Included, Unbounded};
#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W, Q) : (usize, usize, usize),
        RC: [(usize, usize); Q],
    }

    let RC: Vec<(usize, usize)> = RC.into_iter().map(|(r, c)| (r - 1, c - 1)).collect();

    let mut w_tree: Vec<BTreeSet<usize>> = vec![(0..W).into_iter().collect::<BTreeSet<usize>>(); H];
    let mut h_tree: Vec<BTreeSet<usize>> = vec![(0..H).into_iter().collect::<BTreeSet<usize>>(); W];

    let mut cnt = 0;
    for (r, c) in RC.into_iter() {
        let mut bombed = Vec::new();

        // 行方向の破壊した位置
        if let Some(v) = w_tree[r].range((Unbounded, Included(&c))).rev().next() {
            bombed.push((r, *v));
        }
        if let Some(v) = w_tree[r].range((Included(&c), Unbounded)).next() {
            bombed.push((r, *v));
        }

        // 列方向の破壊した位置
        if let Some(v) = h_tree[c].range((Unbounded, Included(&r))).rev().next() {
            bombed.push((*v, c));
        }
        if let Some(v) = h_tree[c].range((Included(&r), Unbounded)).next() {
            bombed.push((*v, c));
        }

        bombed.sort();
        bombed.dedup();

        cnt += bombed.len();
        for (b_r, b_c) in bombed.iter() {
            h_tree[*b_c].remove(b_r);
            w_tree[*b_r].remove(b_c);
        }
    }

    println!("{}", H * W - cnt);
}
