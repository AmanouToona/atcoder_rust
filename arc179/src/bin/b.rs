use ac_library::ModInt1000000007 as Mint;
use proconio::input;
use std::collections::HashMap;

#[allow(non_snake_case)]
fn main() {
    input! {
        M: usize,
        N: usize,
        X: [usize; M],
    }

    let X: Vec<usize> = X.into_iter().map(|x| x - 1).collect();

    // 逆引きの作成
    let mut Y: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, &x) in X.iter().enumerate() {
        Y.entry(x).or_insert(Vec::new()).push(i);
    }

    // dp[i文字目?][右端に挿入できる数字の組み]
    let mut dp = vec![vec![Mint::new(0); 1 << M]; N + 1];
    dp[0][(1 << M) - 1] = Mint::new(1); // 初期状態ではすべてを挿入可能

    // 遷移
    for i in 0..N {
        // println!("\ni: {}", i);
        for nxt in 0..M {
            for state in 1..(1 << M) {
                if state >> nxt & 1 == 0 {
                    continue;
                }

                let mut v_state = state;
                v_state -= 1 << nxt;

                for &ok in Y.entry(nxt).or_default().iter() {
                    if v_state >> ok & 1 == 1 {
                        continue;
                    }

                    v_state += 1 << ok;
                }

                let tmp = dp[i][state];
                // println!("{:#08b}, {:#08b}, {}", state, v_state, tmp);

                dp[i + 1][v_state] += tmp;
            }
        }
    }

    let ans: Mint = dp[N].iter().sum();

    println!("{}", ans);
}
