use ac_library::FenwickTree;
use proconio::input;
fn main() {
    input! {
        (n, q): (usize, usize),
        a: [usize; n],
        q: [(usize, usize, usize); q],
    }

    let mut ft = FenwickTree::new(n, 0);
    for (i, a_) in a.into_iter().enumerate() {
        ft.add(i, a_)
    }

    for (n, p, x) in q.into_iter() {
        if n == 0 {
            ft.add(p, x);
        } else {
            let ans = ft.sum(p..x);
            println!("{}", ans);
        }
    }
}
