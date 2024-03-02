use ac_library::{LazySegtree, MapMonoid, Monoid};
use std::collections::BTreeSet;

use proconio::input;
fn main() {
    input! {
        (n, q): (usize, usize),
        a: [usize; n],
        q : [(usize, usize, usize); q],
    }

    struct M;
    impl Monoid for M {
        type S = BTreeSet<>
    }
}
