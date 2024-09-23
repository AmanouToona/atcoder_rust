use ac_library::Dsu;
use amplify::confinement::Collection;
use proconio::input;
use std::collections::BTreeSet;
#[allow(non_snake_case)]
fn main() {
    input! {
    (N, Q): (usize, usize),
    query: [(usize, usize, usize); Q],
    }

    let mut uf = Dsu::new(N);
    let mut group = Vec::new();
    for i in 0..N {
        let mut tree = BTreeSet::new();
        tree.push(i);
        group.push(tree);
    }

    for q in query.into_iter() {
        if q.0 == 1 {
            let (u, v) = (q.1 - 1, q.2 - 1);

            if !uf.same(u, v) {
                let u = uf.leader(u);
                let v = uf.leader(v);

                uf.merge(u, v);

                let p = uf.leader(u);
                let child = if p != u { u } else { v };

                for i in group[child].clone().iter().rev().take(10) {
                    group[p].push(*i);
                }
            }
        } else if q.0 == 2 {
            let (v, k) = (q.1 - 1, q.2);
            let p = uf.leader(v);

            if group[p].len() < k {
                println!("-1");
            } else {
                let ans = group[p].iter().rev().take(k).last().unwrap() + 1;
                println!("{ans}")
            }
        } else {
            println!("oop");
        }
    }
}
