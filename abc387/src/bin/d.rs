use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;
#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W): (usize, usize),
        S: [Chars; H],
    }

    let mut sh = 0;
    let mut sw = 0;
    let mut gh = 0;
    let mut gw = 0;
    for h in 0..H {
        for w in 0..W {
            if S[h][w] == 'S' {
                sh = h;
                sw = w;
            } else if S[h][w] == 'G' {
                gh = h;
                gw = w;
            }
        }
    }

    let mut dp = vec![vec![vec![usize::MAX; 2]; W]; H];
    let mut q = VecDeque::new();
    for i in 0..2 {
        dp[sh][sw][i] = 0;
        q.push_back((sh, sw, i));
    }

    let d = [(0, 1), (!0, 0), (0, !0), (1, 0)];

    while let Some((uh, uw, uj)) = q.pop_front() {
        for (vj, &(dh, dw)) in d.iter().enumerate() {
            let vj = vj % 2;

            if vj == uj {
                continue;
            }

            let vh = uh.wrapping_add(dh);
            let vw = uw.wrapping_add(dw);

            if vh >= H || vw >= W {
                continue;
            }

            if S[vh][vw] == '#' {
                continue;
            }

            if dp[vh][vw][vj] <= dp[uh][uw][uj] + 1 {
                continue;
            }

            dp[vh][vw][vj] = dp[uh][uw][uj] + 1;

            q.push_back((vh, vw, vj));
        }
    }

    let ans = dp[gh][gw].iter().min().unwrap().clone();

    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{ans}");
    }
}
