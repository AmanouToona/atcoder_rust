use proconio::input;
use std::collections::VecDeque;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        uvw: [(usize, usize, usize); M],
    }

    let mut g = vec![Vec::new(); N];
    for &(u, v, w) in uvw.iter() {
        let u = u - 1;
        let v = v - 1;

        g[u].push((v, w));
        g[v].push((u, w));
    }

    let mut filter = 0;
    for bit in (0..=30).rev() {
        let tmp_filter = filter | 1 << bit;

        let mut q = VecDeque::new();
        let mut used = vec![false; N];
        used[0] = true;

        for &(v, w) in g[0].iter() {
            if used[v] == true {
                continue;
            }
            if w & tmp_filter != 0 {
                continue;
            }
            q.push_back(v);
            used[v] = true;
        }

        while let Some(u) = q.pop_front() {
            for &(v, w) in g[u].iter() {
                if used[v] == true {
                    continue;
                }
                if w & tmp_filter != 0 {
                    continue;
                }

                used[v] = true;
                q.push_back(v);
            }
        }

        if used[N - 1] == true {
            filter = tmp_filter;
        }
    }

    let mut ans = 0;
    for i in 0..=30 {
        ans += 1 << i;
    }

    ans -= filter;
    println!("{ans}");
}
