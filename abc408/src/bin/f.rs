use ac_library::{LazySegtree, MapMonoid, Monoid, Segtree};
use proconio::input;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::usize;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, D, R): (usize, usize, usize),
        H: [usize; N],
    }

    struct M;

    impl Monoid for M {
        type S = usize;
        fn identity() -> Self::S {
            0
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.max(b)
        }
    }

    let mut cnt = Segtree::<M>::new(N);
    let mut used_q: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mut q = BinaryHeap::new();

    for (i, &h) in H.iter().enumerate() {
        q.push((h, i));
    }

    while let Some((h, u)) = q.pop() {
        while !used_q.is_empty() && used_q.front().unwrap().0 >= h + D {
            let (_, v, c) = used_q.pop_front().unwrap();
            cnt.set(v, c);
        }

        let l = u.saturating_sub(R);
        let r = (u + R).min(N - 1);
        let max = cnt.prod(l..=r);
        used_q.push_back((h, u, max + 1));
    }

    while let Some((h, u, c)) = used_q.pop_front() {
        cnt.set(u, c);
    }

    let ans = cnt.prod(0..N) - 1;
    println!("{ans}");
}
