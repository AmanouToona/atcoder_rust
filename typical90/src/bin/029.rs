use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (W, N): (usize, usize),
        LR : [(usize, usize); N],
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

    let mut seg = LazySegtree::<F>::new(W + 1);

    for &(left, right) in LR.iter() {
        let hight = seg.prod(left..=right);
        let hight = hight + 1;

        println!("{}", hight);
        seg.apply_range(left..=right, hight);
    }
}
