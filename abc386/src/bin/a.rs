use proconio::input;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        cards: [usize; 4],
    }

    // 2 種類の数字からできていれば ok
    let mut cnt: HashSet<usize> = HashSet::new();

    for c in cards.into_iter() {
        cnt.insert(c);
    }

    if cnt.len() == 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
