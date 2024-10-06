use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
    }

    if S[S.len() - 3..S.len()] == ['s', 'a', 'n'] {
        println!("Yes");
    } else {
        println!("No");
    }
}
