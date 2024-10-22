use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        S:Chars,
        T:Chars,
    }

    let mut diff = Vec::new();

    for (i, (s, t)) in S.iter().zip(T.iter()).enumerate() {
        let s = *s as i32;
        let t = *t as i32;

        diff.push(s - t);
    }

    let mut X = Vec::new();

    let mut S_now = S.clone();
    for (i, &d) in diff.iter().enumerate() {
        if d <= 0 {
            continue;
        }
        S_now[i] = ((S_now[i] as i32 - d) as u8) as char;
        X.push(S_now.clone());
    }

    for (i, d) in diff.into_iter().enumerate().rev() {
        if d >= 0 {
            continue;
        }
        S_now[i] = ((S_now[i] as i32 - d) as u8) as char;
        X.push(S_now.clone());
    }

    println!("{}", X.len());

    if X.len() == 0 {
        return;
    }

    for x in X.iter() {
        let x: String = x.iter().join("");
        println!("{}", x);
    }
}
