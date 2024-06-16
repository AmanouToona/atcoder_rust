use itertools::Itertools;
use proconio::input;

#[allow(clippy::needless_range_loop)]
#[allow(non_snake_case)]

fn main() {
    input! {
        (N, K): (usize, usize),
        XY: [(usize, usize); N],
    }

    // 前処理 グループ内の2点間距離の最大
    let mut dist = vec![0; (1 << N) + 1];

    for bit in 1..=(1 << N) {
        let mut member = Vec::new();
        for i in 0..N {
            if bit >> i & 1 == 1 {
                member.push(i);
            }
        }

        if member.len() == 1 {
            dist[bit] = 0;
            continue;
        }

        for i in member.iter().combinations(2) {
            let &u = i[0];
            let &v = i[1];

            dist[bit] = dist[bit]
                .max((XY[u].0).abs_diff(XY[v].0).pow(2) + (XY[u].1.abs_diff(XY[v].1).pow(2)));
        }
    }

    // dp[グループの数][グループに含まれる点の集合] = 求められている距離
    let mut dp = vec![vec![usize::MAX; (1 << N) + 1]; K + 1];
    dp[0][0] = 0;

    for k in 1..=K {
        for b in 0..=(1 << N) {
            let mut sub = b;
            while sub != 0 {
                dp[k][b] = dp[k][b].min(dp[k - 1][b - sub].max(dist[sub]));
                sub = (sub - 1) & b;
            }
        }
    }

    println!("{}", dp[K][(1 << N) - 1]);
}
