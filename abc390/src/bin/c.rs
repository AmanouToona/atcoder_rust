use std::usize;

use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W): (usize, usize),
        S: [Chars; H],
    }

    let mut w_min = usize::MAX;
    let mut w_max = 0;
    let mut h_min = usize::MAX;
    let mut h_max = 0;

    for h in 0..H {
        for w in 0..W {
            if S[h][w] != '#' {
                continue;
            }

            w_min = w_min.min(w);
            w_max = w_max.max(w);
            h_min = h_min.min(h);
            h_max = h_max.max(h);
        }
    }

    for h in h_min..=h_max {
        for w in w_min..=w_max {
            if S[h][w] == '.' {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
