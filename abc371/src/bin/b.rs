use proconio::input;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        AB: [(usize, char); M],
    }

    let mut set: HashSet<usize> = HashSet::new();

    for (a, b) in AB.iter() {
        if b == &'F' {
            println!("No");
            continue;
        }

        if set.contains(a) {
            println!("No");
            continue;
        }

        set.insert(*a);
        println!("Yes");
    }
}
