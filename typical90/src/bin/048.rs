use std::collections::BinaryHeap;

use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
        AB: [(usize, usize); N],
    }

    let mut q = BinaryHeap::new();
    for &(a, b) in AB.iter() {
        q.push(b);
        q.push(a - b);
    }

    let mut ans = 0;
    for _ in 0..K {
        if let Some(v) = q.pop() {
            ans += v;
        }
    }

    println!("{}", ans);
}
