use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        LR: [(usize, usize); M],
    }

    let mut cumsum = vec![0i64; N + 2];

    for &(l, r) in LR.iter() {
        cumsum[l] += 1;
        cumsum[r + 1] -= 1;
    }

    for i in 0..(cumsum.len() - 1) {
        cumsum[i + 1] += cumsum[i];
    }

    let mut ans = i64::MAX;

    for &i in cumsum.iter().skip(1).take(N) {
        ans = ans.min(i);
    }

    println!("{ans}");
}
