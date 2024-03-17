//https://atcoder.jp/contests/abc341/tasks/abc341_c
use itertools::iproduct;
use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]

fn main() {
    input! {
        (H, W, _): (usize, usize, usize),
        T: Chars,
        S: [Chars; H],
    }

    let mut ans: usize = 0;

    for (sr, sc) in iproduct!(0..H, 0..W) {
        if S[sr][sc] == '#' {
            continue;
        }
        let (mut r, mut c) = (sr, sc);
        let mut is_ans = true;
        for &t in T.iter() {
            if t == 'L' {
                c = c.wrapping_add(!0);
            } else if t == 'R' {
                c += 1;
            } else if t == 'U' {
                r = r.wrapping_add(!0);
            } else {
                r += 1;
            }
            if S[r][c] == '#' {
                is_ans = false;
                break;
            };
        }
        if is_ans {
            ans += 1;
        }
    }
    println!("{}", ans);
}
