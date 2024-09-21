use itertools::Itertools;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        H: [i64;N],
    }

    let mut q: BinaryHeap<i64> = BinaryHeap::new();

    let mut minus = Vec::new();

    for h in H.iter().rev() {
        let mut res = 0;
        while !q.is_empty() {
            let top = -*q.peek().unwrap();
            if h > &top {
                q.pop();
                res -= 1;
            } else {
                break;
            }
        }

        q.push(-(*h));

        minus.push(res);
    }

    for i in 0..(minus.len() - 1) {
        minus[i + 1] += minus[i];
    }

    let mut ans = Vec::new();
    for i in 0..N {
        ans.push(i as i64);
    }

    for i in 1..N {
        ans[i] += minus[i - 1];
    }

    let ans: String = ans.iter().rev().join(" ");
    println!("{ans}")
}
