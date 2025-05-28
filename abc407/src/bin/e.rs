use proconio::input;
use std::collections::BinaryHeap;
#[allow(non_snake_case)]
fn solve() {
    input! {N: usize}
    input! {A: [usize; 2 * N]}

    let mut q = BinaryHeap::new();
    let mut ans = 0;

    for (i, &a) in A.iter().enumerate() {
        q.push(a);

        if i % 2 == 1 {
            continue;
        }
        ans += q.pop().unwrap();
    }

    println!("{ans}");
}

#[allow(non_snake_case)]
fn main() {
    input! {T: usize}

    for _ in 0..T {
        solve()
    }
}
