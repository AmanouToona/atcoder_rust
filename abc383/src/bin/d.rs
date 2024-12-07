use num_integer::Roots;
use proconio::input;
use std::collections::BTreeSet;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }

    let mut cnt = vec![0; N.sqrt() + 1];
    let mut skip = BTreeSet::new();
    for i in 2..=N.sqrt() {
        let mut j = i;

        skip.insert(j * j);

        if skip.contains(&j) {
            continue;
        }
        cnt[j] += 1;

        while j + i <= N.sqrt() {
            cnt[j + i] += 1;
            j += i;
        }
    }

    let mut ans = 0;

    for i in 2..=N.sqrt() {
        if cnt[i] == 4 {
            ans += 1;
        }
    }

    println!("{:?}", cnt);

    println!("{ans}");
}
