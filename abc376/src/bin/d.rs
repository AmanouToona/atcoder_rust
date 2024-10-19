use proconio::input;
use std::collections::VecDeque;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        ab: [(usize, usize); M],
    }

    let mut g = vec![Vec::new(); N];

    for &(a, b) in ab.iter() {
        let a = a - 1;
        let b = b - 1;

        g[a].push(b);
    }

    let mut q = VecDeque::new();
    let mut cnt = vec![None; N];
    cnt[0] = Some(0);
    q.push_back(0);

    while let Some(u) = q.pop_front() {
        let cnt_u = cnt[u].unwrap();
        for &v in g[u].iter() {
            if v == 0 {
                println!("{}", cnt_u + 1);
                return;
            }
            if cnt[v].is_some() {
                continue;
            }
            cnt[v] = Some(cnt_u + 1);
            q.push_back(v);
        }
    }
    println!("-1");
}
