use itertools::Itertools;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        uvt: [(usize, usize, usize,); M],
        Q: usize,
    }

    // ? 多重編の管理の一般的な方法は?
    // g: (to, cost, edge_num)
    let mut g: Vec<Vec<(usize, usize, usize)>> = vec![Vec::new(); N];

    for (i, &(u, v, t)) in uvt.iter().enumerate() {
        let u = u - 1;
        let v = v - 1;

        g[u].push((v, t, i));
        g[v].push((u, t, i));
    }

    'outer: for _ in 0..Q {
        input! {
            K: usize,
            B: [usize; K],
        }

        let B = B.into_iter().map(|x| x - 1).collect::<Vec<usize>>();
        let mut b2num = HashMap::new();
        for (i, b) in B.iter().enumerate() {
            b2num.insert(*b, i);
        }

        let mut used = HashSet::new();
        let mut q = BinaryHeap::new();
        q.push((Reverse(0), 0, 0)); // cost, node, used b
        used.insert((0, 0)); // node, used b

        let goal = (N - 1, 2usize.pow((K as i32).try_into().unwrap()) - 1);

        while let Some((Reverse(u_cost), u, e)) = q.pop() {
            for (v, diff_cost, e_num) in g[u].iter() {
                let mut e_v = e;
                if b2num.keys().contains(&e_num) {
                    e_v |= 1 << b2num.get(&e_num).unwrap();
                }

                if used.contains(&(*v, e_v)) {
                    continue;
                }

                let v_cost = u_cost + diff_cost;
                if (*v, e_v) == goal {
                    println!("{v_cost}");
                    continue 'outer;
                }

                used.insert((*v, e_v));
                q.push((Reverse(v_cost), *v, e_v));
            }
        }
    }
}
