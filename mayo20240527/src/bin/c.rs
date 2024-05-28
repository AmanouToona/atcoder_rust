use proconio::input;
use proconio::marker::Chars;
// ABC287 D

#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
        T: Chars,
    }

    let mut pre = vec![true];
    for (s, t) in S.iter().zip(T.iter()) {
        if s == t || *s == '?' || *t == '?' {
            pre.push(true);
        } else {
            pre.push(false);
        }
    }

    for i in 0..(pre.len() - 1) {
        if !pre[i] {
            pre[i + 1] = false;
        }
    }

    let mut suf = vec![true];
    for (s, t) in S.iter().rev().zip(T.iter().rev()) {
        if s == t || *s == '?' || *t == '?' {
            suf.push(true);
        } else {
            suf.push(false);
        }
    }

    for i in 0..(suf.len() - 1) {
        if !suf[i] {
            suf[i + 1] = false;
        }
    }

    for (i, j) in pre.iter().zip(suf.iter().take(T.len() + 1).rev()) {
        if i & j {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
