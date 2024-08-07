//https://atcoder.jp/contests/abc349/tasks/abc349_d
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (L, R): (usize, usize),
    }

    let mut ans = Vec::new();
    let mut r = 0;
    let mut l = L;
    while r != R {
        let mut i = 0;
        while l % 2usize.pow(i + 1) == 0 && R - l >= 2usize.pow(i + 1) {
            i += 1;
        }

        r = l + 2usize.pow(i);
        ans.push((l, r));
        l = r;
    }

    println!("{}", ans.len());
    for &(l, r) in ans.iter() {
        println!("{l} {r}");
    }
}
