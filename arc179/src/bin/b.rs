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

    // 逆引きの作成
    let mut b: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, &m) in X.iter().enumerate() {
        b.entry(m).or_insert(Vec::new()).push(i + 1);
    }
    let b = b;

    let mut dp: HashMap<Vec<(i64, i64)>, Mint> = HashMap::new();
    // dp 初期化 i を挿入する
    for i in 1..=M {
        let mut state = vec![(0, 0); M + 1];

        if b.contains_key(&i) {
            for &y in b.get(&i).unwrap().iter() {
                state[y].0 += 1;
                state[y].1 += 1;
            }
        }

        *dp.entry(state).or_insert(Mint::new(0)) += Mint::new(1);
    }

    // dp は長さ1のA 状態を保持した初期状態から開始する

    for _ in 0..(N - 1) {
        let mut vdp: HashMap<Vec<(i64, i64)>, Mint> = HashMap::new();

        for (k, v) in dp.iter() {
            // 左端に i を挿入する
            'left: for i in 1..=M {
                let mut state = k.clone();

                // 消費不能
                if state[i].0 < 0i64 {
                    continue;
                }

                state[i].0 -= 1;

                if b.contains_key(&i) {
                    for &y in b.get(&i).unwrap().iter() {
                        state[y].1 += 1;
                        state[y].1 = state[y].1.max(1);
                        // if state[y].1 > 1 {
                        //     continue 'left;
                        // }
                    }
                }

                *vdp.entry(state).or_insert(Mint::new(0)) += v;
            }

            // 右端に i を挿入する
            'right: for i in 1..=M {
                let mut state = k.clone();

                if state[i].1 < 0i64 {
                    continue;
                }

                state[i].1 -= 1;

                //
                if b.contains_key(&i) {
                    for &y in b.get(&i).unwrap().iter() {
                        state[y].0 += 1;
                        state[y].0 = state[y].0.max(1);
                        // state[y].0 += 1;
                        // if state[y].0 > 1 {
                        //     continue 'right;
                        // }
                    }
                }

                *vdp.entry(state).or_insert(Mint::new(0)) += v;
            }
        }

        dp = vdp;
    }

    let mut ans = Mint::new(0);

    for v in dp.values() {
        ans += v;
    }

    println!("{}", ans);
}
