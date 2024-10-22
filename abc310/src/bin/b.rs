use std::collections::HashSet;

use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
    }

    let mut P = Vec::new();
    let mut C = Vec::new();
    let mut F: Vec<HashSet<usize>> = Vec::new();

    for _ in 0..N {
        input! {
            p: usize,
            c: usize,
            f: [usize; c],
        }

        P.push(p);
        C.push(c);
        F.push(f.into_iter().collect());
    }

    for i in 0..N {
        for j in 0..N {
            if i == j {
                continue;
            }

            if P[i] < P[j] {
                continue;
            }

            if F[i]
                .difference(&F[j])
                .cloned()
                .collect::<Vec<usize>>()
                .len()
                > 0
            {
                continue;
            }

            if P[j] < P[i] {
                println!("Yes");
                return;
            }

            if F[j]
                .difference(&F[i])
                .cloned()
                .collect::<Vec<usize>>()
                .len()
                > 0
            {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
