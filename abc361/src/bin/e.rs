use im_rc::HashMap;
use proconio::input;
use std::{cmp::Reverse, collections::BinaryHeap};
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        ABC: [(usize, usize, usize); N - 1],
    }

    let mut g = vec![vec![]; N];
    let mut dist = HashMap::new();

    for &(a, b, c) in ABC.iter() {
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);

        dist.insert((a - 1, b - 1), c);
        dist.insert((b - 1, a - 1), c);
    }

    let mut q = BinaryHeap::new();
    let mut used = vec![false; N];
    let mut far = 0;
    q.push((Reverse(0), 0));
    used[0] = true;

    while let Some((Reverse(d), u)) = q.pop() {
        far = u;
        for &v in g[u].iter() {
            if used[v] {
                continue;
            }
            q.push((Reverse(d + *dist.get(&(u, v)).unwrap()), v));
            used[v] = true;
        }
    }

    let mut used = vec![false; N];
    let mut q = BinaryHeap::new();

    q.push((Reverse(0), far));
    used[far] = true;
    let mut far = 0;

    while let Some((Reverse(d), u)) = q.pop() {
        far = d;
        for &v in g[u].iter() {
            if used[v] {
                continue;
            }
            q.push((Reverse(d + *dist.get(&(u, v)).unwrap()), v));
            used[v] = true;
        }
    }
    // println!("{far}");

    let mut ans: usize = ABC.iter().map(|x| x.2).sum();
    ans *= 2;
    ans -= far;
    println!("{ans}")
}
