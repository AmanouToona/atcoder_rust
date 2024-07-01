use im_rc::HashSet;
use itertools::Itertools;
//https://atcoder.jp/contests/abc310/tasks/abc310_d
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, T, M): (usize, usize, usize),
        AB: [(usize, usize); M],
    }

    let mut bad: HashSet<(usize, usize)> = HashSet::new();
    for &(a, b) in AB.iter() {
        bad.insert((a - 1, b - 1));
        bad.insert((b - 1, a - 1));
    }

    let mut ok = vec![true; 1 << N];
    'outer: for bit in 0..(1 << N) {
        let mut group = Vec::new();
        for i in 0..N {
            if bit >> i & 1 == 1 {
                group.push(i);
            }
        }

        if group.len() < 2 {
            continue;
        }

        for g in group.iter().combinations(2) {
            if bad.contains(&(*g[0], *g[1])) {
                ok[bit] = false;
                continue 'outer;
            }
        }
    }

    // dp[チーム数][チームに入れた選手の集合 (bit)] = 組み合わせの数
    let mut dp = vec![vec![0; 1 << N]; T + 1];
    for (i, &ok) in ok.iter().enumerate().skip(1) {
        if ok {
            dp[1][i] = 1
        }
    }
    dp[0][0] = 1;

    for t in 2..=T {
        for b in 0..(1 << N) {
            let mut sub = b;
            let mut done = HashSet::new();
            loop {
                if ok[sub] && ok[b - sub] {
                    for t_ in 0..=t {
                        if done.contains(&(t_, sub)) || done.contains(&(t - t_, b - sub)) {
                            continue;
                        }
                        // if t == 2 && b == 3 {
                        //     println!("");
                        //     println!("{} {} {} {}", t_, sub, t - t_, b - sub);
                        //     println!("{} {} {}", dp[t][b], dp[t_][sub], dp[t - t_][b - sub]);
                        //     println!("{:?}", done);
                        // }
                        dp[t][b] += dp[t_][sub] * dp[t - t_][b - sub];
                        done.insert((t_, sub));
                        done.insert((t - t_, b - sub));
                    }
                }
                if sub == 0 {
                    break;
                }
                sub = (sub - 1) & b;
            }
        }
    }

    println!("{}", dp[T][(1 << N) - 1]);
}
