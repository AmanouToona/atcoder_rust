use proconio::input;
use std::collections::VecDeque;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        AB: [(usize, usize); N - 1],
    }

    let mut g = vec![vec![]; N];
    for &(a, b) in AB.iter() {
        let a = a - 1;
        let b = b - 1;
        g[a].push(b);
        g[b].push(a);
    }

    // bfs 2回で直径を探索
    let g = g;
    let mut visited = vec![false; N]; // foot print
    let mut q = VecDeque::new();

    q.push_back(0);
    visited[0] = true;

    let mut far = 0;

    while let Some(u) = q.pop_front() {
        far = u;

        for &v in &g[u] {
            if visited[v] {
                continue;
            }

            visited[v] = true;
            q.push_back(v);
        }
    }

    let mut dist = vec![usize::MAX; N];
    dist[far] = 0;
    q.push_back(far);

    while let Some(u) = q.pop_front() {
        for &v in &g[u] {
            if dist[v] == usize::MAX {
                dist[v] = dist[u] + 1;
                q.push_back(v);
            }
        }
    }

    let ans = dist.iter().max().unwrap() + 1;
    println!("{}", ans);
}
