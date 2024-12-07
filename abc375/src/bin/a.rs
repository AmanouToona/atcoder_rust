use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: Chars,
    }

    if N < 3 {
        println!("0");
        return;
    }

    let mut ans = 0;
    for (i, &s) in S.iter().enumerate().take(N - 2) {
        if S[i + 1] == '.' && S[i + 2] == '#' && s == '#' {
            ans += 1;
        }
    }

    println!("{ans}");
}
