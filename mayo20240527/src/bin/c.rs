use proconio::input;
use proconio::marker::Chars;
// ABC287 D

#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
        T: Chars,
    }

    let mut no = 0;
    for (i, &t) in T.iter().rev().enumerate() {
        let s = S[S.len() - 1 - i];

        if t != s && t != '?' && s != '?' {
            no += 1
        }
    }

    let mut ans = Vec::new();
    ans.push(if no == 0 { "Yes" } else { "No" });

    for (i, &t) in T.iter().enumerate() {
        let s = S[S.len() - (T.len() - i)];

        if s != t && s != '?' && t != '?' {
            no += 1;
        }

        if S[i] != '?' && t != '?' && S[i] != t {
            no += 1;
        }
        ans.push(if no == 0 { "Yes" } else { "No" });
    }

    for a in ans.iter() {
        println!("{}", a);
    }
}
