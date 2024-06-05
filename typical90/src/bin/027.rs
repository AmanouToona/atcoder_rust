use amplify::confinement::Collection;
use proconio::input;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {N : usize}

    let mut s = HashSet::new();

    let mut ans = Vec::new();
    for n in 0..N {
        input! {S: String}
        if s.contains(&S) {
            continue;
        }

        s.push(S);
        ans.push(n + 1);
    }

    for a in ans.iter() {
        println!("{}", a);
    }
}
