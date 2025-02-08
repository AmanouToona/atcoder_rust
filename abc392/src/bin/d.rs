use proconio::input;
use std::collections::HashMap;
#[allow(non_snake_case)]

fn main() {
    input! {
        N: usize,
    }

    let mut a_size = Vec::new();
    let mut nums = Vec::new();
    for _ in 0..N {
        input! {
            K: usize,
            A: [usize; K],
        }

        a_size.push(K);

        let mut num = HashMap::new();

        for &a in A.iter() {
            *num.entry(a).or_insert(0) += 1;
        }
        nums.push(num);
    }

    let mut a1 = 0;
    let mut a2 = 1;

    for i in 0..N {
        for j in (i + 1)..N {
            let b2 = a_size[i] * a_size[j];

            let mut b1 = 0;
            for (k, &v) in nums[i].iter() {
                let v2 = nums[j].get(k).unwrap_or(&0);
                b1 += v * v2;
            }

            if a1 * b2 > b1 * a2 {
                continue;
            }

            a1 = b1;
            a2 = b2;
        }
    }

    let ans = a1 as f64 / a2 as f64;

    println!("{ans:.15}");
}
