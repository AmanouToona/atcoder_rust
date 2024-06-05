use std::collections::BinaryHeap;

use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        lr: [(i64, i64); N],
    }

    let mut lr = lr;
    lr.sort_by(|a, b| a.0.cmp(&b.0));

    let mut ans = 0;
    let mut heap: BinaryHeap<i64> = BinaryHeap::new();

    for &(l, r) in lr.iter() {
        while let Some(&v) = heap.peek() {
            if -v < l {
                heap.pop();
            } else {
                break;
            }
        }

        ans += heap.len();
        heap.push(-r);
    }

    println!("{}", ans);
}
