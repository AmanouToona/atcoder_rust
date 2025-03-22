use proconio::input;
use std::collections::HashMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        A: [usize; 7],
    }

    let mut cnt = HashMap::new();

    for a in A.into_iter() {
        *cnt.entry(a).or_insert(0) += 1;
    }

    let mut over3 = 0;
    let mut over2 = 0;

    for (key, &val) in cnt.iter() {
        if val >= 3 {
            over3 += 1;
        } else if val >= 2 {
            over2 += 1;
        }
    }

    let mut ans = over3;

    if over2 > 0 {
        ans += 1
    }

    if ans >= 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
