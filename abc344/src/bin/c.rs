use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        n: usize,
        a: [isize; n],
        m: usize,
        b: [isize; m],
        l: usize,
        c: [isize; l],
        q: usize,
        x: [isize; q],
    }

    let mut can = HashSet::new();

    for i_a in a.iter() {
        for i_b in b.iter() {
            for i_c in c.iter() {
                can.insert(i_a + i_b + i_c);
            }
        }
    }

    for x_ in x.iter() {
        if can.contains(x_) {
            println!("Yes");
        } else {
            println!("No")
        }
    }
}
