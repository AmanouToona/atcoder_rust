use ac_library::{Monoid, Segtree};
use im_rc::HashMap;
use proconio::input;
use proconio::marker::Chars;
use std::collections::BTreeSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        T: i64,
        S: Chars,
        X: [i64; N],
    }

    let mut ant = Vec::new();
    for (x, s) in X.into_iter().zip(S.into_iter()) {
        ant.push((x, s));
    }

    ant.sort();
    // println!("{:?}", ant);

    let mut after = Vec::new();
    for &(x, s) in ant.iter() {
        let y = if s == '0' { x - T } else { x + T };
        after.push(y);
    }

    // after.reverse();

    // 座標圧縮
    let mut ps = BTreeSet::new();
    for y in after.iter() {
        ps.insert(y);
    }

    let mut conv = HashMap::new();
    for (i, p) in ps.iter().enumerate() {
        conv.insert(p, i);
    }

    // seg木で回答
    struct M;
    impl Monoid for M {
        type S = usize;
        fn identity() -> Self::S {
            0
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a + b
        }
    }

    let mut seg = Segtree::<M>::new(conv.len());
    let n = conv.len();

    let mut ans = 0;

    for y in after.iter() {
        let y = conv.get(&y).unwrap();

        ans += seg.prod(y..&n);

        let cnt = seg.get(*y);
        seg.set(*y, cnt + 1);
    }

    println!("{ans}")
}
