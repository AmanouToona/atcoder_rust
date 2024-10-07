use std::collections::HashSet;

use proconio::input;

struct Info {
    g: Vec<Vec<usize>>,
    V: HashSet<usize>,
    cnt: Vec<usize>,
}

impl Info {
    fn new(g: &Vec<Vec<usize>>, n: usize, v: &Vec<usize>) -> Self {
        Info {
            g: g.clone(),
            V: v.iter().cloned().collect(),
            cnt: vec![0; n],
        }
    }
}

fn dfs(p: usize, u: usize, info: &mut Info) {
    if info.V.contains(&u) {
        info.cnt[u] += 1;
    }
    for &v in info.g[u].clone().iter() {
        if v == p {
            continue;
        }
        dfs(u, v, info);

        info.cnt[u] += info.cnt[v];
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
        AB: [(usize, usize ); N - 1],
        V: [usize; K],
    }

    let mut g = vec![Vec::new(); N];
    for &(a, b) in AB.iter() {
        let a = a - 1;
        let b = b - 1;
        g[a].push(b);
        g[b].push(a);
    }

    let V: Vec<usize> = V.iter().cloned().map(|x| x - 1).collect();

    let mut info = Info::new(&g, N, &V);
    dfs(V[0], V[0], &mut info);

    let mut ans = 0;
    for i in 0..N {
        if info.cnt[i] != 0 {
            ans += 1;
        }
    }

    println!("{ans}");
}
