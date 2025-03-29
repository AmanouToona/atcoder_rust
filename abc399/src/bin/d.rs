use proconio::input;
use std::collections::HashSet;
fn canon(a: usize, b: usize) -> (usize, usize) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {T: usize}

    for _ in 0..T {
        input! {
            N: usize,
            A: [usize ; 2 * N],
        }

        let mut cnt = HashSet::new();
        let mut ans = 0;

        let mut cant = HashSet::new();
        for (&a, &b) in A.iter().zip(A.iter().skip(1)) {
            if a == b {
                cant.insert(a);
            }
        }

        for (&a, &b) in A.iter().zip(A.iter().skip(1)) {
            if cant.contains(&a) || cant.contains(&b) {
                continue;
            }

            if cnt.contains(&(canon(a, b))) {
                ans += 1;
            } else {
                cnt.insert(canon(a, b));
            }
        }

        println!("{ans}");
    }
}
