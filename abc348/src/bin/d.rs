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
        medicines.insert((r - 1, c - 1), e);
    }

    let mut s = (0, 0);
    let mut t = (0, 0);

    for (h, w) in iproduct!(0..H, 0..W) {
        match A[h][w] {
            'S' => s = (h, w),
            'T' => t = (h, w),
            _ => (),
        }
    }

    let mut g = vec![vec![-1; W]; H];
    let mut q = VecDeque::new();

    q.push(s);
    g[s.0][s.1] = *medicines.get(&s).unwrap_or(&0);

    while let Some((uh, uw)) = q.pop_front() {
        if g[uh][uw] == 0 {
            continue;
        }
        for (dh, dw) in [(0, 1), (!0, 0), (0, !0), (1, 0)] {
            let vh = uh.wrapping_add(dh);
            let vw = uw.wrapping_add(dw);

            if vh >= H || vw >= W {
                continue;
            }

            if A[vh][vw] == '#' {
                continue;
            }

            let nxt_e = g[vh][vw]
                .max(g[uh][uw] - 1)
                .max(*medicines.get(&(vh, vw)).unwrap_or(&0));

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
