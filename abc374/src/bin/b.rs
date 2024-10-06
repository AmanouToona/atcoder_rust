use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
        T: Chars,
    }

    for i in 0..S.len().max(T.len()) {
        if S.len() <= i {
            println!("{}", i + 1);
            return;
        } else if T.len() <= i {
            println!("{}", i + 1);
            return;
        } else if S[i] != T[i] {
            println!("{}", i + 1);
            return;
        }
    }
    println!("0");

    // if S.len() > T.len() {
    //     println!("{}", T.len() + 1);
    //     return;
    // } else if S.len() < T.len() {
    //     println!("{}", S.len() + 1);
    //     return;
    // } else {
    //     for (i, (s, t)) in S.iter().zip(T.iter()).enumerate() {
    //         if s != t {
    //             println!("{}", i + 1);
    //             return;
    //         }
    //     }
    // }
    // println!("0");
}
