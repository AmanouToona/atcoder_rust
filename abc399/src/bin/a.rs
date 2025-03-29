use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: Chars,
        T: Chars,
    }

    let mut ans = 0;
    for (&s, &t) in S.iter().zip(T.iter()) {
        if s != t {
            ans += 1;
        }
    }

    println!("{ans}");
}
