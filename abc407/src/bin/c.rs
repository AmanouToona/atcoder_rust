use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
    }

    let mut roll = 0;
    let mut ans = 0;
    for s in S.iter().rev() {
        let mut s = s.to_digit(10).unwrap() as i32;
        s = (s - roll + 10) % 10;

        // 数字 0 を末尾に追加したぶん
        ans += 1;
        ans += s; // 数字を 0 から変更したぶん

        roll += s;
        roll %= 10;
    }
    println!("{}", ans);
}
