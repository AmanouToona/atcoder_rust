use itertools::Itertools;
use proconio::input;
use std::collections::VecDeque;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, W): (usize, usize),
        xy: [(usize, usize); N],
        Q: usize,
        ta: [(usize, usize); Q],
    }

    let xy: Vec<(usize, usize)> = xy
        .into_iter()
        .map(|(x, y)| (x - 1, y))
        .sorted_by_key(|x| x.1)
        .collect();

    // 初期配置を作成
    let mut wait = vec![VecDeque::new(); W];

    for (i, &(x, y)) in xy.iter().enumerate() {
        wait[x].push_back((y, i));
    }

    let mut del_time = vec![usize::MAX; N];
    let mut u_time: usize = !0;
    'outer: loop {
        let mut nxt_time = u_time.wrapping_add(1);

        let mut que = VecDeque::new();
        for w in 0..W {
            if wait[w].is_empty() {
                break 'outer;
            }

            let (top, num) = wait[w].pop_front().unwrap();
            nxt_time = nxt_time.max(top);
            que.push_back(num);
        }

        while let Some(u) = que.pop_front() {
            del_time[u] = nxt_time;
        }
        u_time = nxt_time;
    }

    for &(t, a) in ta.iter() {
        if del_time[a - 1] > t {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
