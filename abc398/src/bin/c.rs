use proconio::input;
use std::collections::HashMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut cnt = HashMap::new();

    for &a in A.iter() {
        *cnt.entry(a).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (&key, &val) in cnt.iter() {
        if val == 1 {
            ans = ans.max(key);
        }
    }

    if ans == 0 {
        println!("-1");
        return;
    }

    for (i, &a) in A.iter().enumerate() {
        if a == ans {
            println!("{}", i + 1);
            return;
        }
    }
}
