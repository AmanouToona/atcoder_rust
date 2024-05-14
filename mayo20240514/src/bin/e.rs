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

    for pair in (0..N).combinations(2) {
        let i = pair[0];
        let j = pair[1];

        // 傾き
        let mut x = XY[j].0 - XY[i].0;
        let mut y = XY[j].1 - XY[i].1;

        if x < 0 {
            x *= -1;
            y *= -1;
        }

        if x == 0 {
            y = 1;
        }

        let gcd = x.gcd(&y);

        x /= gcd;
        y /= gcd;

        // y切片 的な物
        let x1 = XY[i].0;
        let y1 = XY[i].1;

        let b = x * y1 - y * x1;

        *cnt.entry((x, y, b)).or_default() += 1;
    }

    // println!("{:?}", cnt);

    let mut ans = 0;
    for &v in cnt.values() {
        if v >= K * (K - 1) / 2 {
            ans += 1
        }
    }

    println!("{}", ans);
}
