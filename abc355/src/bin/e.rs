use std::collections::VecDeque;

use proconio::input_interactive;

#[allow(non_snake_case)]
fn main() {
    input_interactive!((N, L, R): (usize, usize, usize));

    let mut pre: Vec<Option<usize>> = vec![None; (1 << N) + 2];
    pre[L] = Some(L);
    let mut q: VecDeque<usize> = VecDeque::from([L]);

    while let Some(u) = q.pop_front() {
        for i in 0..=N {
            // 右に移動
            let v = u + (1 << i);
            if v <= (1 << N) && pre[v].is_none() {
                pre[v] = Some(u);
                q.push_back(v);
            }

            // 左に移動
            if u >= (1 << i) {
                let v = u - (1 << i);
                if pre[v].is_none() {
                    pre[v] = Some(u);
                    q.push_back(v);
                }
            }

            if (u >> i) & 1 == 1 {
                break;
            }
        }

        if pre[R + 1].is_some() {
            break;
        }
    }

    let mut r = R + 1;
    let mut ans = 0;
    while r != L {
        let nxt = pre[r].unwrap();

        let sig = if nxt < r { 1 } else { -1 };

        let diff = r.abs_diff(nxt);
        let mut i = 0;

        while (diff >> i) != 1 {
            i += 1;
        }
        let j = nxt.min(r) / (1 << i);

        println!("? {} {}", i, j);

        input_interactive!(a: i64);

        ans += sig * a;
        ans = ans.rem_euclid(100);

        r = nxt;
    }

    println!("! {}", ans);
}
