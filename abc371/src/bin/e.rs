use proconio::input;
use std::collections::HashMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut cnt = Vec::new();
    let mut dict = HashMap::new();
    for a in A.iter() {
        *dict.entry(*a).or_insert(0) += 1;
        cnt.push(dict.len());
    }

    let mut ans = 0;

    for i in 0..cnt.len() {
        let mut in_cnt = cnt.clone();

        let res = in_cnt[i] - 1;
        for j in 0..in_cnt.len() {
            in_cnt[j] = in_cnt[j].saturating_sub(res);
        }
        ans += in_cnt.iter().rev().take(N - i).sum::<usize>();
        println!(
            "{:?} {}",
            in_cnt,
            in_cnt.iter().rev().take(N - i).sum::<usize>()
        );
    }

    println!("{:?}, {}", cnt, ans);
}
