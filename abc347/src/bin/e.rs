use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, Q): (usize,  usize),
        X: [usize; Q],
    }

    let mut cumsum = vec![0];
    let mut S: HashSet<usize> = HashSet::new();
    let mut pos: HashMap<usize, usize> = HashMap::new();

    let mut A = vec![0; N + 1];

    for (i, &x) in X.iter().enumerate() {
        if S.contains(&x) {
            A[x] += cumsum[i] - cumsum[pos[&x]];
            pos.remove(&x);

            S.remove(&x);
            cumsum.push(S.len());
            cumsum[i + 1] += cumsum[i];
        } else {
            S.insert(x);
            cumsum.push(S.len());
            cumsum[i + 1] += cumsum[i];
            pos.insert(x, i);
        }
    }

    for i in 1..=N {
        if !pos.contains_key(&i) {
            continue;
        }
        A[i] += cumsum.last().unwrap() - cumsum[pos[&i]]
    }

    let ans: String = A.into_iter().skip(1).join(" ");
    println!("{}", ans);
}
