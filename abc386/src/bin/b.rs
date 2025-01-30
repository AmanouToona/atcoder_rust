use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
    }

    let mut left = 0;
    let mut cnt = 0;

    while left < S.len() {
        cnt += 1;
        if S[left] == '0' && left + 1 < S.len() && S[left + 1] == '0' {
            left += 2;
        } else {
            left += 1;
        }
    }

    println!("{cnt}");
}
