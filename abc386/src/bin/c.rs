use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
        S: Chars,
        T: Chars,
    }

    if S.len() == T.len() {
        // change
        let mut cnt_diff = 0;
        for (s, t) in S.iter().zip(T.iter()) {
            if s != t {
                cnt_diff += 1;
            }
        }

        if cnt_diff <= 1 {
            println!("Yes");
            return;
        }
    } else if S.len() < T.len() {
        // insert s
        let mut cnt_diff = 0;
        let mut pos = 0;

        for &t in T.iter() {
            if pos >= S.len() {
                cnt_diff += 1;
                continue;
            }
            if t != S[pos] {
                cnt_diff += 1;
            } else {
                pos += 1;
            }
        }

        if cnt_diff <= 1 {
            println!("Yes");
            return;
        }
    } else {
        // insert t
        let mut cnt_diff = 0;
        let mut pos = 0;

        for &s in S.iter() {
            if pos >= T.len() {
                cnt_diff += 1;
                continue;
            }

            if s != T[pos] {
                cnt_diff += 1;
            } else {
                pos += 1;
            }
        }

        if cnt_diff <= 1 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
