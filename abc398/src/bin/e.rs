use proconio::input_interactive;
use std::collections::HashSet;
use std::collections::VecDeque;

fn canon(u: usize, v: usize) -> (usize, usize) {
    if u < v {
        (u, v)
    } else {
        (v, u)
    }
}

#[allow(non_snake_case)]
fn main() {
    input_interactive!(N: usize);
    input_interactive!(UV: [(usize, usize); N - 1]);

    let mut g = vec![Vec::new(); N];

    for &(u, v) in UV.iter() {
        let u = u - 1;
        let v = v - 1;

        g[u].push(v);
        g[v].push(u);
    }

    let mut color = vec![0; N];
    let mut fp = vec![false; N];

    let mut q = VecDeque::new();
    q.push_back(0);

    while let Some(u) = q.pop_front() {
        if fp[u] {
            continue;
        }
        fp[u] = true;

        for &v in g[u].iter() {
            if !fp[v] {
                q.push_back(v);
                color[v] = 1 - color[u];
            }
        }
    }

    let mut connected = HashSet::new();
    for u in 0..N {
        for &v in g[u].iter() {
            connected.insert(canon(u, v));
        }
    }

    let mut c1 = HashSet::new();
    let mut c0 = HashSet::new();

    for (i, &c) in color.iter().enumerate() {
        if c == 0 {
            c0.insert(i);
        } else {
            c1.insert(i);
        }
    }

    let mut unconnected = HashSet::new();
    for &u in c0.iter() {
        for &v in c1.iter() {
            let edge = canon(u, v);
            if !connected.contains(&edge) {
                unconnected.insert(edge);
            }
        }
    }

    fn play_remove(unconnected: &mut HashSet<(usize, usize)>) {
        let &(u, v) = unconnected.iter().next().unwrap();
        println!("{} {}", u + 1, v + 1);
        unconnected.remove(&(u, v));
    }

    if unconnected.len() % 2 == 1 {
        println!("First");
        play_remove(&mut unconnected);
    } else {
        println!("Second");
    }

    loop {
        input_interactive!((u, v): (i64, i64));

        if u < 0 && v < 0 {
            return;
        }
        let u: usize = (u - 1).try_into().unwrap();
        let v: usize = (v - 1).try_into().unwrap();

        let edge = canon(u, v);
        unconnected.remove(&edge);

        play_remove(&mut unconnected);
    }
}
