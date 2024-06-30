use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
        T: Chars,
    }

    let mut s = Vec::new();
    s.push('0');

    for ss in S.iter() {
        s.push(*ss);
    }
    let S = s;

    for w in 1..(S.len() - 1) {
        for s in 1..=w {
            let mut c = Vec::new();
            let mut nxt = s;
            while nxt < S.len() {
                c.push(S[nxt]);
                nxt += w;
            }

            if c == T {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
