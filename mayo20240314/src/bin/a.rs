// https://atcoder.jp/contests/abc201/tasks/abc201_b
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]

fn main() {
    input! {
        N: usize,
        st: [(String, usize); N],
    }

    let mut st = st;
    st.sort_by(|x, y| (y.1).cmp(&x.1));
    println!("{}", st[1].0);
}
