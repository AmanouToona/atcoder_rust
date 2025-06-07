use proconio::input;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: [i64; N],
        uvw: [(usize, usize, usize); N - 1],
    }

    let mut e_cnt = vec![0; N];
    let mut weight = HashMap::new();
    let mut g = vec![HashSet::new(); N];
    let mut xs: Vec<i64> = X.clone();

    for &(u, v, w) in uvw.iter() {
        let u = u - 1;
        let v = v - 1;

        e_cnt[u] += 1;
        e_cnt[v] += 1;

        weight.insert((u, v), w);
        weight.insert((v, u), w);
        g[u].insert(v);
        g[v].insert(u);
    }

    let mut q = VecDeque::new();
    for (i, &cnt) in e_cnt.iter().enumerate() {
        if cnt == 1 {
            q.push_back(i);
        }
    }

    let mut ans: usize = 0;
    while let Some(u) = q.pop_front() {
        let mut v = usize::MAX;
        for &x in g[u].iter() {
            if e_cnt[x] > 0 {
                v = x;
                break;
            }
        }

        if usize::MAX == v {
            continue;
        }

        ans += xs[u].abs() as usize * weight[&(u, v)];

        xs[v] += xs[u];
        xs[u] = 0;
        e_cnt[u] -= 1;
        e_cnt[v] -= 1;

        if e_cnt[v] == 1 {
            q.push_back(v);
        }

        g[v].remove(&u);
    }

    println!("{}", ans);
}
