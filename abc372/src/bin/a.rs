use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
    }

    let ans: String = S.into_iter().filter(|x| *x != '.').collect();
    println!("{}", ans);
}
