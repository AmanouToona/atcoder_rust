use std::collections::HashSet;

use amplify::confinement::Collection;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, T, M): (usize, usize, usize),
        AB: [(usize, usize); M],
    }

    let mut cant_pair = vec![HashSet::new(); N];
    for &(a, b) in AB.iter() {
        let a = a - 1;
        let b = b - 1;

        cant_pair[a].insert(b);
        cant_pair[b].insert(a);
    }

    let mut cant_team = HashSet::new();

    'outer: for i in 0..(1 << N) {
        let mut member = Vec::new();
        for j in 0..N {
            if (i >> j) & 1 == 1 {
                member.push(j);
            }
        }

        for (j, _) in member.iter().enumerate() {
            for m_k in member.iter().skip(j + 1) {
                if cant_pair[member[j]].contains(m_k) {
                    cant_team.insert(i);
                    continue 'outer;
                }
            }
        }
    }

    // dp[チームに用いた集合][チーム数]
    let mut dp = vec![vec![0; T + 1]; 1 << N];
    dp[0][0] = 1;
    for i in 1..1 << N {
        if !cant_team.contains(&i) {
            dp[i][1] = 1;
        }
    }

    for i in 0..1 << N {
        for j in 1..=T {
            let mut sub = i;
            while sub > 0 {
                sub = (sub - 1) & i;
                dp[i][j] += dp[sub][1] * dp[i - sub][j - 1];
            }
        }
    }

    let mut ans = dp[(1 << N) - 1][T];
    for i in 1..=T {
        ans /= i
    }

    println!("{}", ans);
}
