use ac_library::Monoid;
use ac_library::Segtree;
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        A: [usize; N],
    }

    let A: Vec<usize> = A.into_iter().map(|x| x % M).collect();
    let mut cumsum = vec![0; N + 1];
    for (i, &a) in A.iter().enumerate() {
        cumsum[i + 1] = cumsum[i] + a;
        cumsum[i + 1] %= M;
    }

    struct Mono;

    impl Monoid for Mono {
        type S = usize;
        fn identity() -> Self::S {
            0
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a + b
        }
    }

    let mut seg = Segtree::<Mono>::new(M);

    let mut ans = 0;

    let mut sum_l = 0;
    for r in 0..N {
        let sr1 = cumsum[r + 1];

        sum_l += cumsum[r];

        let n = seg.get(cumsum[r]);
        seg.set(cumsum[r], n + 1);

        // 転倒数
        let inversion_num = r + 1 - seg.prod(0..=sr1);

        ans += (r + 1) * sr1 + M * inversion_num - sum_l;
    }

    println!("{ans}");
}
