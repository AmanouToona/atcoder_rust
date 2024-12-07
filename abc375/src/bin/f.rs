use proconio::input;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M, Q): (usize, usize, usize),
        ABC: [(usize, usize, usize); M],
    }

    // クエリの読み方 保存のしかたがわからない
    let mut closed = HashSet::new();

    let mut query = Vec::new();
    for _ in 0..Q {
        input! {q: usize}
        if q == 1 {
            input! {
                i: usize
            }
            query.push(vec![1, i]);
            closed.insert(i);
        } else {
            input! {
                (x, y): (usize, usize)
            }
            query.push(vec![2, x, y]);
        }
    }

    // クエリ逆順処理
    // 通行止めになる道以外でワーシャルフロイド

    let mut dp = vec![vec![usize::MAX; N]; N];
    for i in 0..N {
        dp[i][i] = 0;
    }

    for (i, &(a, b, c)) in ABC.iter().enumerate() {
        let a = a - 1;
        let b = b - 1;

        // 最終状態で通行止め
        if closed.contains(&(i + 1)) {
            continue;
        }

        dp[a][b] = c;
        dp[b][a] = c;
    }

    for k in 0..N {
        for u in 0..N {
            for v in 0..N {
                if dp[u][k] == usize::MAX || dp[k][v] == usize::MAX {
                    continue;
                }
                dp[u][v] = dp[u][v].min(dp[u][k] + dp[k][v]);
            }
        }
    }

    let mut ans = Vec::new();
    for q in query.iter().rev() {
        if q.len() == 2 {
            // 使えるようになった通路を追加
            let (a, b, c) = ABC[q[1] - 1];
            let a = a - 1;
            let b = b - 1;

            dp[a][b] = dp[a][b].min(c);
            dp[b][a] = dp[b][a].min(c);

            for u in 0..N {
                for v in 0..N {
                    if dp[u][a] != usize::MAX && dp[b][v] != usize::MAX {
                        dp[u][v] = dp[u][v].min(dp[u][a] + dp[b][v] + dp[a][b]);
                    }
                    if dp[u][b] != usize::MAX && dp[a][v] != usize::MAX {
                        dp[u][v] = dp[u][v].min(dp[u][b] + dp[a][v] + dp[a][b]);
                    }
                    dp[v][u] = dp[u][v]
                }
            }
        } else if q.len() == 3 {
            let x = q[1] - 1;
            let y = q[2] - 1;

            if dp[x][y] != usize::MAX {
                ans.push(dp[x][y] as i64);
            } else {
                ans.push(-1);
            }
        } else {
            println!("miss !!");
        }
    }

    for i in ans.iter().rev() {
        println!("{i}");
    }
}
