use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }

    // 到達可能な右端を seg で管理
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
    struct F;
    impl MapMonoid for F {
        type M = M;
        type F = usize;

        fn identity_map() -> Self::F {
            0
        }
        fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
            *f.max(x)
        }
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            *f.max(g)
        }
    }

    for i in 0..N {
        seg.apply_range(i..=i, i);
    }

    let mut seg = LazySegtree::<F>::new(W + 1);

    for _ in 0..Q {
        input! {q: usize}

        if q == 1 {
            input! {(x, c): (usize, usize)}

            let x = x - 1;
            let c = c - 1;
        } else if q == 2 {
            input! {c: usize}
        } else {
            println!("strange input !!!");
        }
    }
}
