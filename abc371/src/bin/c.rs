use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        MG: usize,
        UV: [(usize, usize); MG],
        MH: usize,
        AB: [(usize, usize); MH],
    }

    let mut A = Vec::new();
    for i in 0..N - 1 {
        input! {a: [usize; N - i -  1]};
        A.push(a);
    }

    let mut cost = HashMap::new();
    for (i, a) in A.iter().enumerate() {
        for (j, aa) in a.iter().enumerate() {
            cost.insert((i, i + j + 1), *aa);
            cost.insert((i + j + 1, i), *aa);
        }
    }

    // 前処理
    let UV: Vec<(usize, usize)> = UV.into_iter().map(|x| (x.0 - 1, x.1 - 1)).collect();
    let AB: Vec<(usize, usize)> = AB.into_iter().map(|x| (x.0 - 1, x.1 - 1)).collect();

    // 結合の初期化
    let mut g_connected = HashSet::new();
    for (u, v) in UV.iter() {
        g_connected.insert((*u, *v));
        g_connected.insert((*v, *u));
    }

    // 最小コスト探索
    let mut ans = 10usize.pow(9u32);
    for i in (0..N).permutations(N) {
        let mut converter = HashMap::new();
        let mut re_converter = HashMap::new();
        for (from, to) in i.iter().enumerate() {
            converter.insert(from, *to);
            re_converter.insert(to, from);
        }

        // コスト候補探索
        let mut in_cost = 0;

        let mut h_connected = HashSet::new();
        for (a, b) in AB.iter() {
            let a2 = converter.get(a).unwrap();
            let b2 = converter.get(b).unwrap();

            if !g_connected.contains(&(*a2, *b2)) {
                in_cost += *cost.get(&(*a, *b)).unwrap();
            }
            h_connected.insert((*a2, *b2));
            h_connected.insert((*b2, *a2));
        }

        for (u, v) in UV.iter() {
            if !h_connected.contains(&(*u, *v)) {
                let u = re_converter.get(u).unwrap();
                let v = re_converter.get(v).unwrap();
                in_cost += *cost.get(&(*u, *v)).unwrap();
            }
        }
        ans = ans.min(in_cost);
    }

    println!("{ans}");
}
