use itertools::Itertools;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        H : [usize; N],
    }

    let mut ans = Vec::new();
    let mut q = BinaryHeap::new();

    for h in H.into_iter().rev() {
        ans.push(q.len());

        while !q.is_empty() && q.peek().unwrap() > &Reverse(h) {
            q.pop();
        }

        q.push(Reverse(h));
    }

    ans = ans.into_iter().rev().collect();

    let ans: String = ans.iter().join(" ");
    println!("{ans}");
}
