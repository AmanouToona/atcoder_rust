use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
    }

    for &s in S.iter() {
        if s == 'R' {
            println!("Yes");
            return;
        } else if s == 'M' {
            println!("No");
            return;
        }
    }
}
