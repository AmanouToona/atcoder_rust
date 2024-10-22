use itertools::iproduct;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W, Y): (usize, usize, usize),
        A: [[usize; W]; H],
    }

    let mut q: BinaryHeap<(Reverse<usize>, usize, usize)> = BinaryHeap::new();
    let mut used = vec![vec![false; W]; H];

    // 外周部分を q に入れる
    for (h, w) in iproduct!(0..H, 0..W) {
        if h == 0 || h == H - 1 || w == 0 || w == W - 1 {
            q.push((Reverse(A[h][w]), h, w));
            used[h][w] = true;
        }
    }

    let mut sink = vec![0; Y + 1];
    let mut level = 0;
    while let Some((Reverse(a), h, w)) = q.pop() {
        level = level.max(a);
        if level > Y {
            break;
        }
        sink[level] += 1;

        for (&dh, &dw) in [0, !0, 0, 1].iter().zip([1, 0, !0, 0].iter()) {
            let vh = h.wrapping_add(dh);
            let vw = w.wrapping_add(dw);

            if vh >= H || vw >= W {
                continue;
            }

            if used[vh][vw] {
                continue;
            }

            q.push((Reverse(A[vh][vw]), vh, vw));
            used[vh][vw] = true;
        }
    }

    for i in 0..Y {
        sink[i + 1] += sink[i];
    }

    for &s in sink.iter().skip(1) {
        println!("{}", H * W - s);
    }
}
