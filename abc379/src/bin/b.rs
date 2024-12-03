use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
        S: Chars,
    }

    let mut ans = 0;
    let mut cnt = 0;

    for &s in S.iter() {
        if s == 'X' {
            cnt = 0;
            continue;
        }

        cnt += 1;

        if cnt >= K {
            ans += 1;
            cnt = 0;
        }
    }

    println!("{ans}");
}
