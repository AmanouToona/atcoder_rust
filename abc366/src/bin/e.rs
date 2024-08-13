use proconio::input;
use std::collections::HashMap;

#[allow(non_snake_case)]
fn f(X: Vec<i32>) -> Vec<usize> {
    let mut pos = HashMap::new();
    for &x in X.iter() {
        *pos.entry(x.clone()).or_default() += 1;
    }

    // 位置 -2 * 10^6 での Sigma |x - xi|
    let band = 2 * 10_i32.pow(6);
    let mut sum: i32 = 0;
    for &x in X.iter() {
        sum += x.abs_diff(-band) as i32;
    }

    // 位置を -2 * 10^6 -> 2 * 10^6 に操作しながら距離和を計算する
    let mut over_i = X.len() as i32;
    let mut under_i = 0;
    let mut res = Vec::new();
    for i in (-band)..(band) {
        // 位置 i での距離の差を保存
        res.push(sum.try_into().unwrap());

        // 位置 i+ 1 での距離の差を計算する
        over_i -= pos.get(&i).unwrap_or(&0);
        under_i += pos.get(&i).unwrap_or(&0);
        sum += -over_i + under_i;
    }

    res.sort();
    res
}

#[allow(non_snake_case)]
fn main() {
    input! {(N, D): (usize, usize),
    mut XY: [(i32, i32);N],
    }

    let X = XY.iter().map(|x| x.0).collect();
    let X: Vec<usize> = f(X).into_iter().rev().collect();

    let Y = XY.iter().map(|x| x.1).collect();
    let Y = f(Y);

    let mut ans = 0;
    let mut right = 0; // 尺取り法用
    for x in X.iter() {
        while right < Y.len() && Y[right] + x <= D {
            right += 1;
        }
        ans += right;
    }

    println!("{ans}");
}
