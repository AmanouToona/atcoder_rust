use ac_library::Dsu;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        clr: [(usize, usize, usize); M],
    }

    let mut h = BinaryHeap::new();
    for &(c, l, r) in clr.iter() {
        let l = l - 1;
        h.push((Reverse(c), l, r));
    }

    let mut uf = Dsu::new(N + 1);
    let mut ans = 0;
    while let Some((Reverse(c), l, r)) = h.pop() {
        if uf.same(l, r) {
            continue;
        }
        uf.merge(l, r);

        ans += c;
    }

    // 結合不能な点が残っている?
    for i in 0..=N {
        if !uf.same(0, i) {
            println!("-1");
            return;
        }
    }

    println!("{ans}");
}
