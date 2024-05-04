use std::collections::BinaryHeap;

use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        ABC: [(usize, usize, usize); M],
    };

    let mut g = vec![vec![]; N + 1];

    for &(a, b, c) in ABC.iter() {
        g[a].push((b, c));
        g[b].push((a, c));
    }

    // dijkstra を 1 start と N start の計2回行う
    let mut from1 = vec![usize::MAX; N + 1];
    from1[1] = 0;
    let mut q: BinaryHeap<(i64, usize)> = BinaryHeap::new();

    q.push((0, 1));

    while let Some((_, u)) = q.pop() {
        for &(v, c) in g[u].iter() {
            let nxt_cost = from1[u] + c;

            if from1[v] <= nxt_cost {
                continue;
            }

            from1[v] = nxt_cost;

            q.push((-(nxt_cost as i64), v));
        }
    }

    // N start
    let mut fromN = vec![usize::MAX; N + 1];
    fromN[N] = 0;
    let mut q: BinaryHeap<(i64, usize)> = BinaryHeap::new();

    q.push((0, N));

    while let Some((_, u)) = q.pop() {
        for &(v, c) in g[u].iter() {
            let nxt_cost = fromN[u] + c;

            if fromN[v] <= nxt_cost {
                continue;
            }

            fromN[v] = nxt_cost;

            q.push((-(nxt_cost as i64), v));
        }
    }

    // 回答出力
    for k in 1..=N {
        let ans = from1[k] + fromN[k];
        println!("{}", ans);
    }
}
