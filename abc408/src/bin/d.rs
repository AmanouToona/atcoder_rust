use std::i64;

use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
    }

    for _ in 0..T {
        input! {
            N: usize,
            S: Chars,
        }

        let mut A = vec![0; N + 1]; // [0, i) を 1 にするのにかかるコスト
        let mut B = vec![0; N + 1]; // [0, i) を 0 にするのにかかるコスト
        for (i, &s) in S.iter().enumerate() {
            if s == '0' {
                A[i + 1] += 1;
            } else {
                B[i + 1] += 1;
            }
        }

        for i in 0..N {
            A[i + 1] += A[i];
            B[i + 1] += B[i];
        }

        let mut C = Vec::new();
        for (a, b) in A.iter().zip(B.iter()) {
            C.push(*a - *b);
        }

        // println!("{:?}", A);
        // println!("{:?}", B);
        // println!("{:?}", C);

        let mut ans = i64::MAX;
        let mut max_c = 0;
        for c in C.iter() {
            max_c = max_c.max(*c);

            let tmp = c - max_c + B[N];
            ans = ans.min(tmp);
        }
        println!("{ans}");
    }
}
