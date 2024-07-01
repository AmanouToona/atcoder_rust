use im_rc::HashSet;
use itertools::Itertools;
//https://atcoder.jp/contests/abc232/tasks/abc232_c
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        AB: [(usize, usize); M],
        CD: [(usize, usize); M],
    }

    let mut ab = HashSet::new();
    for (a, b) in AB.into_iter() {
        ab.insert((a - 1, b - 1));
        ab.insert((b - 1, a - 1));
    }

    for i in (0..N).permutations(N) {
        let mut cd = HashSet::new();
        for &(c, d) in CD.iter() {
            cd.insert((i[c - 1], i[d - 1]));
            cd.insert((i[d - 1], i[c - 1]));
        }

        if ab == cd {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
