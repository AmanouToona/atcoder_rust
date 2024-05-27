use itertools::iproduct;
use proconio::input;
use proconio::marker::Chars;
// ABC312 B

#[allow(non_snake_case)]
fn main() {
    // # black, . white
    input! {
        (N, M): (usize, usize),
        S: [Chars; N],
    }

    let mut ans = Vec::new();
    'outer: for (top, left) in iproduct!(0..(N - 8), 0..(M - 8)) {
        // is brack
        for (r, c) in iproduct!(0..3, 0..3) {
            if S[top + r][left + c] != '#' {
                continue 'outer;
            }
        }
        for (r, c) in iproduct!(6..9, 6..9) {
            if S[top + r][left + c] != '#' {
                continue 'outer;
            }
        }

        // is white
        for r in 0..4 {
            if S[top + r][left + 3] != '.' {
                continue 'outer;
            }
        }
        for c in 0..4 {
            if S[top + 3][left + c] != '.' {
                continue 'outer;
            }
        }
        for r in 5..9 {
            if S[top + r][left + 5] != '.' {
                continue 'outer;
            }
        }
        for c in 5..9 {
            if S[top + 5][left + c] != '.' {
                continue 'outer;
            }
        }

        ans.push((top + 1, left + 1));
    }

    for (i, j) in ans.iter() {
        println!("{} {}", i, j);
    }
}
