// https://atcoder.jp/contests/abc319/tasks/abc319_c
use itertools::Itertools;
use proconio::input;

fn is_ok(v: &Vec<usize>) -> bool {
    if v.len() != 2 {
        return true;
    }

    if v[0] == v[1] {
        return false;
    }

    true
}

#[allow(non_snake_case)]
fn main() {
    input! {
        C: [usize; 9],
    }

    let mut ok = 0;
    let mut ng = 0;

    'outer: for i in (0..9).permutations(9) {
        let mut fp = vec![vec![false; 3]; 3];

        for j in i.iter().take(8) {
            let h = j / 3;
            let w = j % 3;

            fp[h][w] = true;

            // 横の確認
            for h in 0..3 {
                let mut num = Vec::new();
                for w in 0..3 {
                    if fp[h][w] {
                        num.push(C[h * 3 + w]);
                    }
                }
                if !is_ok(&num) {
                    ng += 1;
                    continue 'outer;
                }
            }

            // 縦の確認
            for w in 0..3 {
                let mut num = Vec::new();
                for h in 0..3 {
                    if fp[h][w] {
                        num.push(C[h * 3 + w]);
                    }
                }

                if !is_ok(&num) {
                    ng += 1;
                    continue 'outer;
                }
            }

            // 斜めの確認
            let mut num = Vec::new();
            for p in 0..3 {
                if fp[p][p] {
                    num.push(C[p * 3 + p]);
                }
            }

            if !is_ok(&num) {
                ng += 1;
                continue 'outer;
            }

            let mut num = Vec::new();
            for p in 0..3 {
                if fp[p][2 - p] {
                    num.push(C[p * 3 + (2 - p)]);
                }
            }

            if !is_ok(&num) {
                ng += 1;
                continue 'outer;
            }
        }
        ok += 1;
    }

    // println!("{} {}", ok, ng);

    let ans: f64 = ok as f64 / (ok + ng) as f64;
    println!("{}", ans);
}
