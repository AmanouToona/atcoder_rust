use ac_library::Dsu;
use proconio::input;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        ab : [(usize, usize); M],
    }
    let ab: Vec<(usize, usize)> = ab.into_iter().map(|(a, b)| (a - 1, b - 1)).collect();

    let mut uf1 = Dsu::new(N);
    for &(a, b) in ab.iter() {
        uf1.merge(a, b);
    }

    let mut leaders = HashSet::new();
    for n in 0..N {
        leaders.insert(uf1.leader(n));
    }

    // println!("{:?}", leaders);
    let mut ans = Vec::new();

    let mut uf2 = Dsu::new(N);
    for (i, &(a, b)) in ab.iter().enumerate() {
        if !uf2.same(a, b) {
            uf2.merge(a, b);
            continue;
        }

        if leaders.len() <= 1 {
            continue;
        }

        let la = uf1.leader(a);
        let mut lb = N;
        for &i in leaders.iter() {
            if i == la {
                continue;
            } else {
                lb = i;
                break;
            }
        }

        uf2.merge(a, lb);

        uf1.merge(la, lb);

        let rem = if uf1.leader(la) == la { lb } else { la };

        leaders.remove(&rem);

        ans.push((i, a, lb));
    }

    println!("{}", ans.len());
    for &(i, j, k) in ans.iter() {
        println!("{} {} {}", i + 1, j + 1, k + 1);
    }

    // println!("{:?}", ans);
}
