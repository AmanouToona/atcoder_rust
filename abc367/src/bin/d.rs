use proconio::input;
use std::collections::BTreeMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        A: [usize; N],
    }

    let mut cusum = vec![0; 2 * N];
    cusum[0] = A[0] % M;

    for i in 1..cusum.len() {
        cusum[i] += (cusum[i - 1] + A[i % N]) % M;
    }

    //  解答
    let mut cnt: BTreeMap<usize, usize> = BTreeMap::new();
    for &c in cusum.iter().take(N - 1) {
        *cnt.entry(c).or_default() += 1;
    }

    let mut ans = 0;
    for (&t, &s) in cusum.iter().skip(N - 1).take(N).zip(cusum.iter()) {
        *cnt.entry(t).or_default() += 1;
        ans += *cnt.get(&s).unwrap_or(&1) - 1;

        if let Some(val) = cnt.get_mut(&s) {
            *val -= 1;

            if *val == 0 {
                cnt.remove(&s);
            }
        }
    }

    println!("{ans}");
}
