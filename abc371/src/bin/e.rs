use proconio::input;
use std::collections::HashMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut position: HashMap<usize, Vec<usize>> = HashMap::new();

    for (i, &a) in A.iter().enumerate() {
        position.entry(a).or_insert_with(|| vec![0]).push(i + 1);
    }

    for (k, v) in position.iter_mut() {
        v.push(N + 1);
    }

    let mut ans = 0;

    for pos in position.values() {
        ans += N * (N + 1) / 2;

        for i in 0..pos.len() - 1 {
            let left = pos[i];
            let right = pos[i + 1];

            let num = right - left - 1;
            ans -= num * (num + 1) / 2;
        }
    }

    println!("{ans}");
}
