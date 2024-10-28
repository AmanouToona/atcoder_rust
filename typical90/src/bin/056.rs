use proconio::input;
use std::collections::BTreeMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, S): (usize, usize),
        AB: [[usize; 2]; N],
    }

    let mut dp: Vec<Vec<bool>> = vec![vec![false; S + 1]; N + 1];

    dp[0][0] = true;

    for (i, ab) in AB.iter().enumerate() {
        for s in 0..S {
            if dp[i][s] == false {
                continue;
            }
            for j in 0..=1 {
                let nxt = s + ab[j];

                if nxt > S {
                    continue;
                }

                dp[i + 1][nxt] = true;
            }
        }
    }

    if dp[N][S] == false {
        println!("Impossible");
        return;
    }

    let mut ans = Vec::new();

    let mut now = S;
    for before_day in (0..N).rev() {
        'inner: for j in 0..2 {
            if now < AB[before_day][j] {
                continue;
            }

            let pre = now - AB[before_day][j];
            if dp[before_day][pre] == true {
                if j == 0 {
                    ans.push('A');
                } else {
                    ans.push('B');
                }
                now = pre;
                break 'inner;
            }
        }
    }

    let ans: String = ans.iter().rev().collect();
    println!("{ans}");
}
