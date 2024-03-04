use ac_library::{Monoid, Segtree};
use std::collections::BTreeMap;

use proconio::input;
fn main() {
    input! {
        (n, q): (usize, usize),
        a: [usize; n],
        q : [(usize, usize, usize); q],
    }

    struct M;

    impl Monoid for M {
        type S = (isize, usize, isize, usize);
        fn identity() -> Self::S {
            // (1番目に大きい数, 1番目に大きい数の個数, 2番目に大きい数, 2番目に多き数の個数)
            (-1, 0, -2, 0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            let mut cnt: BTreeMap<isize, usize> = BTreeMap::new();

            *cnt.entry(a.0).or_insert(0) += a.1;
            *cnt.entry(a.2).or_insert(0) += a.3;
            *cnt.entry(b.0).or_insert(0) += b.1;
            *cnt.entry(b.2).or_insert(0) += b.3;

            let mut iter = cnt.into_iter().rev();

            let (first, fcnt) = iter.next().unwrap_or((-1, 0));
            let (second, scnt) = iter.next().unwrap_or((-2, 0));
            (first, fcnt, second, scnt)
        }
    }

    let mut segtree = Segtree::<M>::new(n);

    for (i, item) in a.iter().enumerate() {
        segtree.set(i, (*item as isize, 1, -2, 0));
    }

    for (q, p, x) in q {
        if q == 1 {
            let p = p - 1;
            segtree.set(p, (x as isize, 1, -2, 0));
        } else {
            let l = p - 1;
            let r = x - 1;
            let ans = segtree.prod(l..=r);
            println!("{}", ans.3);
        }
    }
}
