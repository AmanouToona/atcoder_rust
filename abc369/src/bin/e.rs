use itertools::Itertools;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        uvt: [(usize, usize, usize,); M],
        Q: usize,
    }

    // ワーシャルフロイド
    let mut dp = vec![vec![10usize.pow(15u32); N]; N];
    for &(u, v, t) in uvt.iter() {
        let u = u - 1;
        let v = v - 1;

        dp[u][v] = dp[u][v].min(t);
        dp[v][u] = dp[v][u].min(t);
    }
    for i in 0..N {
        dp[i][i] = 0;
    }

    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j]);
            }
        }
    }

    // 辺の管理
    let mut num2edge = HashMap::new();
    for (i, &(u, v, _)) in uvt.iter().enumerate() {
        let u = u - 1;
        let v = v - 1;

        num2edge.insert(i, (u, v));
    }

    for _ in 0..Q {
        input! {
            K: usize,
            B: [usize; K],
        }

        let B: Vec<usize> = B.into_iter().map(|x| x - 1).collect();

        let mut ans = 10usize.pow(15u32);
        for b in B.iter().permutations(K) {
            for direction in 0..(1 << K) {
                let mut now = 0;
                let mut can = 0;
                for i in 0..K {
                    let (from, to) = if (direction >> i) & 1 == 0 {
                        *num2edge.get(&b[i]).unwrap()
                    } else {
                        let &(v, u) = num2edge.get(&b[i]).unwrap();
                        (u, v)
                    };

                    can += dp[now][from];
                    can += uvt[*b[i]].2;

                    now = to;
                }
                can += dp[now][N - 1];
                ans = ans.min(can);
            }
        }
        println!("{ans}");
    }
}
