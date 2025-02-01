use std::usize;

use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        mut xyc: [(usize, usize, char); M],
    }

    // x(縦) 方向の検査
    xyc.sort_by_key(|&(x, y, _)| x);

    let mut min_w = usize::MAX;
    for &(x, y, c) in xyc.iter() {
        if c == 'W' {
            min_w = min_w.min(y);
        } else {
            if y >= min_w {
                println!("No");
                return;
            }
        }
    }

    // y(横) 方向の検査
    xyc.sort_by_key(|&(_, y, _)| y);

    let mut min_w = usize::MAX;
    for &(x, y, c) in xyc.iter() {
        if c == 'W' {
            min_w = min_w.min(x);
        } else {
            if x >= min_w {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
