use proconio::input;
use proconio::marker::Chars;
use std::mem;
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
        mut S: Chars,
        mut T: Chars,
    }

    if S.len() == T.len() {
        if S.iter().zip(T.iter()).all(|(s, t)| s == t) {
            println!("Yes");
            return;
        }
    }

    if S.len() > T.len() {
        mem::swap(&mut S, &mut T);
    }

    // T の方が長い
    let mut max_l = 0;
    for &s in S.iter() {
        if T[max_l] != s {
            break;
        }
        max_l += 1;
    }

    let mut min_r = T.len();
    for &s in S.iter().rev() {
        if T[min_r - 1] != s {
            break;
        }
        min_r -= 1;
    }

    let len_r = T.len() - min_r;

    if max_l + len_r >= S.len() - 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
