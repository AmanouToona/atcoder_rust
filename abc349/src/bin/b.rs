use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
#[allow(non_snake_case)]
fn main() {
    input! {S: Chars}

    let mut cnt = HashMap::new();

    for s in S.iter() {
        *cnt.entry(s).or_insert(0) += 1;
    }

    let mut cnt_appere = HashMap::new();

    for v in cnt.values() {
        *cnt_appere.entry(v).or_insert(0) += 1;
    }

    for &v in cnt_appere.values() {
        if v != 0 && v != 2 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
