use ac_library::ModInt998244353 as Mint;
use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        (N, Q): (usize, usize),
        A: [Mint; N],

    }

    struct M;
    impl Monoid for M {
        type S = (Mint, Mint); // (ans , データサイズ)

        fn identity() -> Self::S {
            (Mint::new(0), Mint::new(1))
        }

        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            (a.0 + b.0, a.1 + b.1)
        }
    }

    struct F;

    impl MapMonoid for F {
        type M = M;
        type F = (Mint, Mint);

        fn identity_map() -> Self::F {
            (Mint::new(1), Mint::new(0))
        }
        fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
            (x.0 * f.0 + x.1 * f.1, x.1)
        }
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            (f.0 * g.0, g.1 * f.0 + f.1)
        }
    }

    let mut seg = LazySegtree::<F>::new(N);
    for (i, a) in A.into_iter().enumerate() {
        seg.set(i, (a, Mint::new(1)));
    }

    for _ in 0..Q {
        input! {
            q: usize,
        }

        if q == 0 {
            input! {
                (l, r, b, c): (usize, usize, Mint, Mint)
            }

            seg.apply_range(l..r, (b, c));
        } else {
            input! {
                (l, r): (usize, usize)
            }

            let ans = seg.prod(l..r);
            println!("{}", ans.0);
        }
    }
}
