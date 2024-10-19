use proconio::input;
use std::collections::BinaryHeap;
#[allow(non_snake_case)]
fn main() {
    input! {T: usize}

    for _ in 0..T {
        solve();
    }
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        (N, K): (usize, usize),
        A: [usize; N],
        B: [usize; N],
    }

    let mut ab: Vec<(usize, usize)> = Vec::new();
    for (&a, &b) in A.iter().zip(B.iter()) {
        ab.push((a, b));
    }

    ab.sort_by_key(|x| x.1);
    ab.sort_by_key(|x| x.0);

    let mut q = BinaryHeap::new();
    let mut sum = 0;
    for &(_, b) in ab.iter().take(K) {
        sum += b;
        q.push(b);
    }

    let mut ans = ab[K - 1].0 * sum;

    for &(a, b) in ab.iter().skip(K) {
        let trash = q.pop().unwrap();
        sum += b;
        sum -= trash;

        ans = ans.min(sum * a);
        q.push(b);
    }

    println!("{ans}");
}
