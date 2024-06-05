use proconio::input;
use std::usize;

fn count0(right: i64, i: usize) -> usize {
    let mut res = right / 4;

    let mut n = res * 4;

    while n <= right {
        if n.rem_euclid(4) as usize == i {
            res += 1;
        }
        n += 1;
    }

    res as usize
}

fn count(left: i64, right: i64, i: usize) -> usize {
    let sub = if left < 0 {
        (-left / 4 + 1) * 4
    } else {
        (left / 4 + 1) * 4
    };

    count0(right + sub, i) - count0(left + sub, i)
}
#[allow(non_snake_case)]
fn main() {
    input! {
    (A, B, C, D): (i64, i64, i64, i64),
    };

    // 面積1
    let mut c1 = 0;
    for (x, y) in [(1, 1), (1, 3), (2, 2), (2, 0)] {
        c1 += count(A, C, x) * count(B, D, y);
    }

    // 面積 0
    let mut c0 = 0;
    for (x, y) in [(3, 1), (3, 3), (0, 2), (0, 0)] {
        c0 += count(A, C, x) * count(B, D, y);
    }

    //
    let ans = c1 as i64 * 2 + (C - A) * (D - B) - (c1 + c0) as i64;
    println!("{}", ans);
}
