use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;

#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W): (usize, usize),
        s: (usize, usize),
        g: (usize, usize),
        S: [Chars; H],
    }

    let s = (s.0 - 1, s.1 - 1);
    let g = (g.0 - 1, g.1 - 1);

    // dp[h][w][direction] = cost
    let mut dp = vec![vec![vec![usize::MAX; 4]; W]; H];
    let mut q = VecDeque::new();
    for i in 0..4 {
        q.push_back((s.0, s.1, i));
        dp[s.0][s.1][i] = 0;
    }

    let d = [(0, 1), (!0, 0), (0, !0), (1, 0)];
    let d_costs = vec![0, 1, 1, 1];
    while !q.is_empty() {
        let (h, w, direction) = q.pop_front().unwrap();

        for (i, &(dh, dw)) in d.iter().cycle().skip(direction).take(4).enumerate() {
            let vh = h.wrapping_add(dh);
            let vw = w.wrapping_add(dw);

            if vh >= H || vw >= W || S[vh][vw] == '#' {
                continue;
            }

            let vc = dp[h][w][direction] + d_costs[i];
            let vd = (direction + i) % 4;

            if dp[vh][vw][vd] <= vc {
                continue;
            }

            dp[vh][vw][vd] = vc;
            q.push_back((vh, vw, vd));
        }
    }

    let ans = dp[g.0][g.1].iter().min().unwrap();
    println!("{}", ans);
}
