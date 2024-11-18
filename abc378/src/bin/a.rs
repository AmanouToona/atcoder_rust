use proconio::input;
use std::collections::HashMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        A: [usize; 4],
    }

    let mut cnt = HashMap::new();

    for &a in A.iter() {
        *cnt.entry(a).or_insert(0) += 1;
    }

    let mut ans = 0;
    for v in cnt.values() {
        ans += v / 2;
    }

    println!("{ans}");
}
