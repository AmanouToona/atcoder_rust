use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        S: [Chars; 12],
    }

    let mut ans = 0;
    for (i, s) in S.iter().enumerate() {
        if s.len() == i + 1 {
            ans += 1;
        }
    }

    println!("{ans}");
}
