use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        T: i64,
        S: Chars,
        X: [i64; N],
    }

    let mut go_r = Vec::new(); // 右に向かう蟻の初期位置
    let mut go_l = Vec::new(); // 左に向かう蟻の初期位置

    for (&x, &s) in X.iter().zip(S.iter()) {
        if s == '0' {
            go_l.push(x);
        } else {
            go_r.push(x);
        }
    }

    go_l.sort();
    go_r.sort();

    // 尺取り法
    let mut left = 0;
    let mut right = 0;
    let mut ans = 0;

    for &r in go_r.iter() {
        let pos = r + 2 * T;

        while left < go_l.len() && go_l[left] < r {
            left += 1;
        }
        while right < go_l.len() && go_l[right] <= pos {
            right += 1;
        }

        ans += right - left;
    }

    println!("{ans}")
}
