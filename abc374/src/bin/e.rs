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

            // 安い方で変えるだけ買う
            let mut in_cost = mid.div_ceil(&a) * p;

            // 高い方を買う
            for i in 1..=100 {
                in_cost = in_cost.min(mid.saturating_sub(i * b).div_ceil(&a) * p + i * q);
            }

            // 高い方も買いつつ調整
            // for res in 1..=100.min(mid) {
            //     in_cost = in_cost.min((mid - res).div_ceil(&a) * p + res.div_ceil(&b) * q);
            // }
            cost += in_cost;

            // cost += (mid.div_ceil(&a) * p).min(mid.div_ceil(&b) * q)
        }

        if cost <= X {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{ok}");
}
