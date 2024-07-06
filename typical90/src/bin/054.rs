use std::collections::VecDeque;

use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
    }

    let mut g = vec![Vec::new(); N + M];

    for m in 0..M {
        input! {
            K: usize,
            R: [usize; K],
        }

        for r in R.into_iter() {
            let u = r - 1;
            let v = N + m;
            g[v].push(u);
            g[u].push(v);
        }
    }

    // 最短経路探索
    let mut depth = vec![usize::MAX; N + M];
    let mut q = VecDeque::new();
    depth[0] = 0;
    q.push_back(0);

    while let Some(u) = q.pop_front() {
        for &v in g[u].iter() {
            if depth[v] != usize::MAX {
                continue;
            };
            q.push_back(v);
            depth[v] = depth[u] + 1;
        }
    }

    for &d in depth.iter().take(N) {
        if d == usize::MAX {
            println!("-1");
        } else {
            println!("{}", d / 2);
        }
    }
}
