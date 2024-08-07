//https://atcoder.jp/contests/abc344/tasks/abc344_c
use proconio::input;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        M: usize,
        B: [usize; M],
        L: usize,
        C:[usize; L] ,
        Q: usize,
        X:[usize; Q],
    }

    let mut set = HashSet::new();

    for &a in A.iter() {
        for &b in B.iter() {
            for &c in C.iter() {
                set.insert(a + b + c);
            }
        }
    }

    for x in X.iter() {
        if set.contains(x) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
