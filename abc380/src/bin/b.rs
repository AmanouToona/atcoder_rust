use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
    }

    let mut A = Vec::new();

    let mut cnt = 0;
    for &s in S.iter() {
        if s == '|' {
            A.push(cnt);
            cnt = 0;
        } else {
            cnt += 1;
        }
    }

    let ans: String = A.iter().skip(1).join(" ");
    println!("{ans}");
}
