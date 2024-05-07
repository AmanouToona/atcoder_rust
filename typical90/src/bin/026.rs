use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        AB: [(usize, usize); N - 1],
    }

    let mut g: Vec<Vec<usize>> = vec![vec![]; N];

    for &(a, b) in AB.iter() {
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }

    let mut fp = vec![0; N]; // 0: 未踏, 1: selected, 2 unselected
    let mut q = VecDeque::new();
    q.push_back(0);
    fp[0] = 1;

    while let Some(u) = q.pop_front() {
        for &v in g[u].iter() {
            if fp[v] != 0 {
                continue;
            }
            q.push_back(v);
            fp[v] = 3 - fp[u];
        }
    }

    let cnt1 = fp.iter().filter(|&&c| c == 1).count();

    let target = if cnt1 >= N / 2 { 1 } else { 2 };
    let ans: Vec<usize> = fp
        .iter()
        .enumerate()
        .filter_map(|(i, f)| if *f == target { Some(i + 1) } else { None })
        .take(N / 2)
        .collect();

    let ans = ans.iter().join(" ");
    println!("{}", ans);
}
