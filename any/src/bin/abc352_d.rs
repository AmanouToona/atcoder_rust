use proconio::input;
use std::collections::BTreeSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
        P: [usize; N],
    }

    let mut idx: Vec<usize> = (0..N).collect();
    idx.sort_by(|x, y| P[*x].cmp(&P[*y]));

    let mut q: BTreeSet<usize> = idx.iter().take(K).cloned().collect();

    let mut ans = q.last().unwrap() - q.iter().next().unwrap();
    for i in 0..N - K {
        q.remove(&idx[i]);
        q.insert(idx[i + K]);

        let can = q.last().unwrap() - q.iter().next().unwrap();
        ans = ans.min(can);
    }

    println!("{ans}");
}
