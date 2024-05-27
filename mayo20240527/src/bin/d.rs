use std::collections::BinaryHeap;

use proconio::input;
// ABC320 E

#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        TWS: [(i64, i64, i64); M],
    }

    let mut wait: BinaryHeap<(i64, i64)> = BinaryHeap::new();
    let mut inline: BinaryHeap<i64> = BinaryHeap::new();

    for i in 0..N as i64 {
        inline.push(-i);
    }

    let mut ans = vec![0; N];
    for &(t, w, s) in TWS.iter() {
        while let Some(&v) = wait.peek() {
            if -v.0 > t {
                break;
            }
            wait.pop();
            inline.push(-v.1);
        }

        if let Some(&v) = inline.peek() {
            let u = -v as usize;
            ans[u] += w;
            inline.pop();
            wait.push((-(t + s), u as i64));
        }
    }

    for a in ans {
        println!("{}", a);
    }
}
