use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;
#[allow(non_snake_case)]

fn main() {
    input! {
        (H, W, D) : (usize, usize, usize),
        S: [Chars; H],
    }

    let mut cnt = vec![vec![usize::MAX; W]; H];
    let mut q = VecDeque::new();

    for i in 0..H {
        for j in 0..W {
            if S[i][j] == 'H' {
                q.push_back((i, j));
                cnt[i][j] = 0;
            }
        }
    }

    while !q.is_empty() {
        let (ui, uj) = q.pop_front().unwrap();

        let nxt_cnt = cnt[ui][uj] + 1;

        if nxt_cnt > D {
            continue;
        }

        for (&di, &dj) in [0, !0, 0, 1].iter().zip([1, 0, !0, 0].iter()) {
            let vi = ui.wrapping_add(di);
            let vj = uj.wrapping_add(dj);

            if vi >= H || vj >= W {
                continue;
            }

            if S[vi][vj] != '.' {
                continue;
            }

            if cnt[vi][vj] != usize::MAX {
                continue;
            }

            cnt[vi][vj] = nxt_cnt;
            q.push_back((vi, vj));
        }
    }

    let mut ans = 0;

    for i in 0..H {
        for j in 0..W {
            if cnt[i][j] != usize::MAX {
                ans += 1;
            }
        }
    }

    println!("{ans}");
}
