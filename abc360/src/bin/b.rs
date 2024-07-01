use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
        T: Chars,
    }

    for w in 1..S.len() {
        for s in 1..=w {
            let mut c = Vec::new();
            let mut nxt = s;

            while nxt <= S.len() {
                c.push(S[nxt - 1]);
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
