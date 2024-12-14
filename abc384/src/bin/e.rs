use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W, X) : (usize, usize, i128),
        (P, Q): (usize, usize),
        S: [[i128; W]; H],
    }

    let mut used = vec![vec![false; W]; H];

    let mut strength = S[P - 1][Q - 1];
    used[P - 1][Q - 1] = true;

    let mut q = BinaryHeap::new();

    let dh: [usize; 4] = [0, 1, 0, !0];
    let dw: [usize; 4] = [1, 0, !0, 0];

    for (&dh, &dw) in dh.iter().zip(dw.iter()) {
        let h = P - 1;
        let w = Q - 1;

        let h = h.wrapping_add(dh);
        let w = w.wrapping_add(dw);

        if h >= H || w >= W {
            continue;
        }

        let s = S[h][w];
        q.push((Reverse(s), h, w));
        used[h][w] = true;
    }

    while let Some((Reverse(s), h, w)) = q.pop() {
        if s * X >= strength {
            continue;
        }

        strength += s;

        for (&dh, &dw) in dh.iter().zip(dw.iter()) {
            let v_h = h.wrapping_add(dh);
            let v_w = w.wrapping_add(dw);

            if v_h >= H || v_w >= W {
                continue;
            }
            if used[v_h][v_w] {
                continue;
            }

            used[v_h][v_w] = true;

            q.push((Reverse(S[v_h][v_w]), v_h, v_w));
        }
    }

    println!("{strength}");
}
