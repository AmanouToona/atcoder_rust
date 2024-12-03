use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: Chars,
    }
    let ans1: String = N.iter().cycle().skip(1).take(3).collect();
    let ans2: String = N.iter().cycle().skip(2).take(3).collect();
    println!("{ans1} {ans2}");
}
