use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
        S: Chars,
        T: Chars,
    }

    if S.len().abs_diff(T.len()) > 1 {
        println!("No");
        return;
    }

    let mut cnt_diff = 0;
    let mut pos_s = 0;
    // change
    if S.len() == T.len() {
        for &t in T.iter() {
            if S[pos_s] != t {
                cnt_diff += 1;
            }
            pos_s += 1;
        }
        if cnt_diff <= 1 {
            println!("Yes");
            return;
        }
    }

    // insert
    cnt_diff = 0;
    pos_s = 0;
    if S.len() < T.len() {
        for &t in T.iter() {
            if pos_s >= S.len() {
                break;
            }
            if S[pos_s] != t {
                cnt_diff += 1;
            } else {
                pos_s += 1;
            }
        }
        if cnt_diff <= 1 {
            println!("Yes");
            return;
        }
    }

    // elase
    cnt_diff = 0;
    pos_s = 0;
    if S.len() > T.len() {
        for &t in T.iter() {
            if S[pos_s] != t {
                cnt_diff += 1;
            }
            pos_s += 1;
        }
        if cnt_diff + pos_s - S.len() <= 1 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
