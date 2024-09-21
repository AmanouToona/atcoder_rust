use amplify::confinement::Collection;
use proconio::input;
use proconio::marker::Chars;
use std::collections::BTreeSet;
#[allow(non_snake_case)]
fn main() {
    input! {
            (N, Q): (usize, usize),
          mut   S: Chars,
    XC: [(usize, char); Q],
        }

    let mut cnt = 0;

    for i in 0..(S.len() - 2) {
        if S[i..(i + 3)] == ['A', 'B', 'C'] {
            cnt += 1;
        }
    }

    for (x, c) in XC.iter() {
        let x = x - 1;
        for i in 0..3 {
            if x < i {
                continue;
            }

            let start = x - i;

            if start + 3 > S.len() {
                continue;
            }

            if S[start..(start + 3)] == ['A', 'B', 'C'] {
                cnt -= 1;
            }
        }

        S[x] = *c;

        for i in 0..3 {
            if x < i {
                continue;
            }

            let start = x - i;

            if start + 3 > S.len() {
                continue;
            }

            if S[start..(start + 3)] == ['A', 'B', 'C'] {
                cnt += 1;
            }
        }

        println!("{cnt}");
    }
}
