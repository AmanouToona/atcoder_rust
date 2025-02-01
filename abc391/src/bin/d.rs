use proconio::input;
use std::collections::VecDeque;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, W): (usize, usize),
        mut xy: [(usize, usize); N],
        Q: usize,
        ta: [(usize, usize); Q],
    }

    // 初期配置を作成
    let mut wait = vec![VecDeque::new(); W];
    xy.sort_by_key(|x| x.1);

    for (i, &(x, y)) in xy.iter().enumerate() {
        wait[x - 1].push_back((y - 1, i));
    }

    let mut del_time = vec![usize::MAX; N];
    'outer: loop {
        let mut nxt_time = 0;

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
            del_time[u] = nxt_time + 1;
        }
    }

    eprintln!("{:?}", del_time);

    for &(t, a) in ta.iter() {
        if del_time[a - 1] > t {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
