// https://atcoder.jp/contests/abc248/tasks/abc248_e
use im_rc::HashMap;
use itertools::Itertools;
use num::Integer;
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
        XY: [(i64, i64); N],
    }

    let inf = "Infinity";

    if K == 1 {
        println!("{}", inf);
        return;
    }

    // 傾き & 切片
    let mut cnt: HashMap<(i64, i64, i64), usize> = HashMap::new();

    for pair in (0..N).permutations(2) {
        let i = pair[0];
        let j = pair[1];

        // 傾き
        let x = XY[j].0 - XY[i].0;
        let y = XY[j].1 - XY[i].1;

        let gcd = x.gcd(&y);

        let mut a1 = x / gcd;
        let mut a2 = y / gcd;

        if a1 < 0 {
            a1 *= -1;
            a2 *= -1;
        }

        if a1 == 0 && a2 < 0 {
            a2 = 1;
        }

        if a2 == 0 {
            a1 = 1;
        }

        // y切片 的な物
        let x1 = XY[i].0;
        let y1 = XY[i].1;

        let b = a1 * y1 - a2 * x1;

        *cnt.entry((a1, a2, b)).or_default() += 1;
    }

    // println!("{:?}", cnt);

    let mut ans = 0;
    for &v in cnt.values() {
        if v > K * (K - 1) / 2 {
            ans += 1
        }
    }

    println!("{}", ans);
}
