use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, Q): (usize,  usize),
        X: [usize; Q],
    }

    let mut cumsum = vec![0];
    let mut pos: HashMap<usize, usize> = HashMap::new();

    let mut A = vec![0; N + 1];

    for (i, &x) in X.iter().enumerate() {
        if let Some(&last_pos) = pos.get(&x) {
            A[x] += cumsum[i] - cumsum[last_pos];
            pos.remove(&x);

            cumsum.push(pos.len() + cumsum[i]);
        } else {
            pos.insert(x, i);
            cumsum.push(pos.len() + cumsum[i]);
        }
    }

    for (&key, &val) in pos.iter() {
        A[key] += cumsum.last().unwrap() - cumsum[val];
    }

    let ans: String = A.into_iter().skip(1).join(" ");
    println!("{}", ans);
}
