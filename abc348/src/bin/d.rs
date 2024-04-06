use amplify::confinement::Collection;
use itertools::iproduct;
use proconio::input;
use proconio::marker::Chars;
use std::collections::{HashMap, VecDeque};
#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W) : (usize, usize),
        A: [Chars; H],
        N: usize,
        RCE: [(usize, usize, isize); N],
    }

    let mut medicines: HashMap<(usize, usize), isize> = HashMap::new();
    for (r, c, e) in RCE.into_iter() {
        medicines.push(((r - 1, c - 1), e));
    }

    let mut s = (0, 0);
    let mut t = (0, 0);

    for (h, w) in iproduct!(0..H, 0..W) {
        if A[h][w] == 'S' {
            s = (h, w);
        } else if A[h][w] == 'T' {
            t = (h, w);
        }
    }

    let mut g = vec![vec![-1; W]; H];
    let mut q = VecDeque::new();

    if !medicines.contains_key(&s) {
        println!("No");
        return;
    }

    q.push(s);
    g[s.0][s.1] = *medicines.get(&s).unwrap();

    while let Some((uh, uw)) = q.pop_front() {
        for (dh, dw) in [(0, 1), (!0, 0), (0, !0), (1, 0)] {
            let vh = uh.wrapping_add(dh);
            let vw = uw.wrapping_add(dw);

            if vh >= H || vw >= W {
                continue;
            }

            if A[vh][vw] == '#' {
                continue;
            }

            let mut nxt_e = g[vh][vw].max(g[uh][uw] - 1);

            if medicines.contains_key(&(vh, vw)) {
                nxt_e = nxt_e.max(*medicines.get(&(vh, vw)).unwrap());
            }

            if nxt_e <= g[vh][vw] {
                continue;
            }

            q.push_back((vh, vw));
            g[vh][vw] = nxt_e;
        }
    }

    if g[t.0][t.1] >= 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
