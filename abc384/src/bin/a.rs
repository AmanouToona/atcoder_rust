use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        c1: char,
        c2 : char,
        S: Chars,
    }

    let mut ans = Vec::new();
    for &s in S.iter() {
        if s == c1 {
            ans.push(s);
        } else {
            ans.push(c2);
        }
    }

    let ans: String = ans.iter().collect();
    println!("{ans}");
}
