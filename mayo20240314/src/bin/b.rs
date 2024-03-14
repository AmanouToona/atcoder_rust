// https://atcoder.jp/contests/abc195/tasks/abc195_c
use proconio::input;
#[allow(non_snake_case)]

fn main() {
    input! {N: usize}

    let mut ans: usize = 0usize;

    for i in (3..19).step_by(3) {
        ans += N.saturating_sub(10usize.pow(i) - 1);
    }

    println!("{}", ans);
}
