use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::input;
fn main() {
    input! {
        (n, q) : (usize, usize),
        a : [usize; n],
        tlr: [(usize, usize, usize); q],
    }

    struct M;

    impl Monoid for M {
        type S = (usize, usize, usize);
        fn identity() -> Self::S {
            // (0の個数, 1の個数, 転倒数)
            (0, 0, 0)
        }
        fn binary_operation(&(a, b, c): &Self::S, &(d, e, f): &Self::S) -> Self::S {
            (a + d, b + e, c + f + d * b)
        }
    }

    struct F;

    // identity_map: lazy data の初期値
    // mapping: lazy -> 下のdata の伝搬
    // composition: lazy -> 下の lazy に流す

    // set: 末端の値を蹴って
    // push lazy を下の data に伝搬させる
    // push の中では、 app_apply が呼ばれており、 中でmapping, composition

    impl MapMonoid for F {
        type M = M;
        type F = bool;

        fn identity_map() -> Self::F {
            // 0 の個数, 1 の個数
            false
        }

        fn mapping(
            &a: &Self::F,
            &(c, d, e): &<Self::M as ac_library::Monoid>::S,
        ) -> <Self::M as ac_library::Monoid>::S {
            if a {
                (d, c, c * d - e)
            } else {
                (c, d, e)
            }
        }

        fn composition(&f: &Self::F, &g: &Self::F) -> Self::F {
            f ^ g
        }
    }

    let mut seg = LazySegtree::<F>::new(n);
    for (i, &a) in a.iter().enumerate() {
        seg.set(i, (1 - a, a, 0))
    }

    for &(t, l, r) in tlr.iter() {
        let l = l - 1;
        let r = r - 1;
        if t == 1 {
            seg.apply_range(l..=r, true)
        } else {
            let (_, _, ans) = seg.prod(l..=r);
            println!("{}", ans);
        }
    }
}
