use proconio::input;
use proconio::marker::Chars;
const INFINITY: usize = usize::MAX; // usize::MAXを無限大の値として使用
#[allow(non_snake_case)]

fn main() {
    input! {
        T: Chars,
        n: usize,
    }

    let mut A = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            a: usize,
            s: [String; a],
        }
        A.push(s);
    }

    let mut dp = vec![INFINITY; T.len() + 1];
    dp[0] = 0;

    for a in A.iter() {
        let mut v_dp = dp.clone();
        for (i, &cost) in dp.iter().enumerate() {
            if cost == INFINITY {
                continue;
            }

            for s in a.iter() {
                if i + s.len() > T.len() {
                    continue;
                }

                if s.chars().zip(&T[i..i + s.len()]).all(|(s, t)| s == *t) {
                    v_dp[i + s.len()] = v_dp[i + s.len()].min(v_dp[i] + 1);
                }
            }
        }
        dp = v_dp;
    }

    if *dp.last().unwrap() != INFINITY {
        println!("{}", dp.last().unwrap());
    } else {
        println!("-1");
    }
}
