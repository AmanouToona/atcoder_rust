use ac_library::{Monoid, Segtree};
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N:usize,
        Q: usize,
        A: [usize; N],
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

    let mut seg = Segtree::<M>::new(N + 1);
    for (i, a) in A.into_iter().enumerate() {
        seg.set(i + 1, a);
    }

    for _ in 0..Q {
        input! {
            (T, X, V) : (usize, usize, usize)
        };

        if T == 1 {
            seg.set(X, V);
        } else if T == 2 {
            let ans = seg.prod(X..=V);
            println!("{}", ans);
        } else if T == 3 {
            if seg.get(X) >= V {
                println!("{}", X);
                continue;
            }
            let mut ok = N + 1;
            let mut ng = X;

            while ok - ng > 1 {
                let mid = (ok + ng) / 2;

                if seg.prod(ng..=mid) >= V {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }

            println!("{}", ok);
        } else {
            println!("error!! {}", T);
            return;
        }
    }
}
