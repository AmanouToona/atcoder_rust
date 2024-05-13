use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (W, N): (usize, usize),
        LRV: [(usize, usize,usize); N],
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
            *x.max(f)
        }
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            *g.max(f)
        }
    }

    let mut dp = vec![0; W + 1];
    for &(l, r, v) in LRV.iter() {
        let mut seg = LazySegtree::<F>::new(W + 1);

        for (i, &u_val) in dp.iter().enumerate() {
            if i != 0 && u_val == 0 {
                continue;
            }
            let left = i + l;
            let right = (i + r).min(W);

            if left > W {
                break;
            }

            seg.apply_range(left..=right, u_val + v)
        }

        for (i, d) in dp.iter_mut().enumerate() {
            *d = *d.max(&mut seg.get(i));
        }
    }

    if dp[W] == 0 {
        println!("-1");
    } else {
        println!("{}", dp[W]);
    }
}
