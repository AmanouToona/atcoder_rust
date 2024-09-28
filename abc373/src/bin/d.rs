use amplify::confinement::Collection;
use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;
use std::collections::VecDeque;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        uvw: [(usize, usize,i64); M],
    }

    let mut g = vec![Vec::<usize>::new(); N];
    let mut g_inv = vec![Vec::<usize>::new(); N];
    let mut w_ij = HashMap::new();
    for (u, v, w) in uvw.iter() {
        let u = *u - 1;
        let v = *v - 1;
        w_ij.insert((u, v), w);
        w_ij.insert((v, u), w);

        g[u].push(v);
        g_inv[v].push(u);
    }

    let mut state: Vec<Option<i64>> = vec![None; N];
    let mut q = VecDeque::new();

    for i in 0..N {
        if state[i] != None {
            continue;
        }
        state[i] = Some(0);
        q.push_back(i);

        while let Some(u) = q.pop_front() {
            for v in g[u].iter() {
                if state[*v] != None {
                    continue;
                }
                state[*v] = Some(state[u].unwrap() + *w_ij.get(&(u, *v)).unwrap());
                q.push_back(*v);
            }

            for v in g_inv[u].iter() {
                if state[*v] != None {
                    continue;
                }

                state[*v] = Some(state[u].unwrap() - *w_ij.get(&(u, *v)).unwrap());
                q.push(*v);
            }
        }
    }

    let mut ans: Vec<i64> = state.iter().map(|x| x.unwrap()).collect();
    let mut ans: String = ans.iter().join(" ");

    println!("{ans}");
}
