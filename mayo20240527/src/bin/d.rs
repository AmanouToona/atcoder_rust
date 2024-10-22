use std::cmp::Reverse;
use std::collections::BinaryHeap;

use proconio::input;
// ABC320 E

#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        TWS: [(usize, usize, usize); M],
    }

    let mut wait: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
    let mut inline = BinaryHeap::new();

    for i in 0..N {
        inline.push(Reverse(i));
    }

    let mut ans = vec![0; N];
    for &(t, w, s) in TWS.iter() {
        while let Some(&(Reverse(t_wait), u_wait)) = wait.peek() {
            if t_wait > t {
                break;
            }
            wait.pop();
            inline.push(Reverse(u_wait));
        }

        if let Some(&Reverse(u)) = inline.peek() {
            ans[u] += w;
            inline.pop();
            wait.push((Reverse(t + s), u));
        }
    }

    for a in ans {
        println!("{}", a);
    }
}
