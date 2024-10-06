use num::Integer;
use proconio::input;
use std::mem::swap;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, X): (usize, usize),
        apbq: [(usize, usize, usize, usize);N],
    }

    let mut ok = 0;
    let mut ng = 10usize.pow(11u32);

    while ng - ok > 1 {
        let mid = (ok + ng) / 2;

        let mut cost = 0;
        for j in 0..N {
            let (mut a, mut p, mut b, mut q) = apbq[j].clone();

            if (p as f64) / (a as f64) >= (q as f64) / (b as f64) {
                swap(&mut a, &mut b);
                swap(&mut p, &mut q);
            }

            let mut in_cost = 10usize.pow(11u32);
            // 機械 1 を買う個数を走査
            for i in 0..=b {
                in_cost = in_cost.min(i * p + mid.saturating_sub(i * a).div_ceil(&b) * q);
            }

            // 機械 2 を買う個数を走査
            for i in 0..=a {
                in_cost = in_cost.min(mid.saturating_sub(i * b).div_ceil(&a) * p + i * q);
            }

            cost += in_cost;
        }

        if cost <= X {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{ok}");
}
