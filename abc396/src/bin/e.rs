use itertools::Itertools;
use proconio::input;
use std::collections::VecDeque;

#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        XYZ: [(usize, usize, usize); M],
    }

    let mut g: Vec<Vec<(usize, usize)>> = vec![Vec::new(); N];

    for &(x, y, z) in XYZ.iter() {
        let x = x - 1;
        let y = y - 1;

        g[x].push((y, z));
        g[y].push((x, z));
    }

    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

    let mut A = vec![0; N];
    for bit in 0..64 {
        let mut a = vec![usize::MAX; N];

        for u in 0..N {
            let mut fp = VecDeque::new();
            let mut cnt1 = 0;
            let mut cnt0 = 0;

            if a[u] != usize::MAX {
                continue;
            }
            fp.push_back(u);
            a[u] = 0;

            cnt0 += 1;

            for &(v, z) in g[u].iter() {
                let av = a[u] ^ (z >> bit & 1);

                if a[v] != usize::MAX && av != a[v] {
                    println!("-1");
                    return;
                }

                if a[v] != usize::MAX {
                    continue;
                }

                a[v] = av;
                q.push_back((v, z));
                fp.push_back(v);

                if av == 1 {
                    cnt1 += 1;
                } else {
                    cnt0 += 1;
                }
            }

            while let Some((u, _)) = q.pop_front() {
                for &(v, z) in g[u].iter() {
                    let av = a[u] ^ (z >> bit & 1);

                    if a[v] != usize::MAX && av != a[v] {
                        println!("-1");
                        return;
                    }

                    if a[v] != usize::MAX {
                        continue;
                    }

                    a[v] = av;

                    if av == 1 {
                        cnt1 += 1;
                    } else {
                        cnt0 += 1;
                    }
                    fp.push_back(v);
                    q.push_back((v, z));
                }
            }

            if cnt0 >= cnt1 {
                while let Some(u) = fp.pop_front() {
                    A[u] += a[u] << bit;
                }
            } else {
                while let Some(u) = fp.pop_front() {
                    A[u] += (1 - a[u]) << bit;
                }
            }
        }
    }

    let ans: String = A.iter().join(" ");
    println!("{ans}");
}
