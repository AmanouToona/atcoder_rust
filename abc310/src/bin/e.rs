use proconio::input;
use proconio::marker::Chars;
use std::mem::swap;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: Chars,
    }

    let S: Vec<usize> = S
        .into_iter()
        .map(|x| if x == '0' { 0 } else { 1 })
        .collect();

    let mut cnt_0: i64 = 0;
    let mut cnt_1: i64 = 0;
    let mut ans: i64 = 0;

    for &s in S.iter() {
        if s == 0 {
            cnt_1 += cnt_0;
            cnt_0 = 1;
        } else {
            swap(&mut cnt_0, &mut cnt_1);
            cnt_1 += 1;
        }
        ans += cnt_1;
    }

    println!("{ans}");
}
