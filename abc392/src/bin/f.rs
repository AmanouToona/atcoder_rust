use ac_library::LazySegtree;
use ac_library::{MapMonoid, Monoid};
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P:[usize; N],
    }

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

    struct F;
    impl MapMonoid for F {
        type M = M;
        type F = usize;

        fn identity_map() -> Self::F {
            0
        }

        fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
            f + x
        }

        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            f + g
        }
    }

    let mut sg = LazySegtree::<F>::new(N + 1);

    let mut ans = vec![0; N];
    for (i, &p) in P.iter().enumerate().rev() {
        sg.apply_range(p..N, 1);
        // println!("{:?}", sg.get(p));
        println!("{}", sg.get(p));
        // ans[sg.get(p) - 1 + p] = i;
    }

    println!("{ans:?}");
}
