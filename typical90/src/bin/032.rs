use im_rc::HashSet;
use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [[usize; N]; N],
        M: usize,
        XY: [(usize, usize); M],
    }

    let mut cant: HashMap<usize, HashSet<usize>> = HashMap::new();

    for &(x, y) in XY.iter() {
        let x = x - 1;
        let y = y - 1;
        cant.entry(x).or_insert(HashSet::new()).insert(y);
        cant.entry(y).or_insert(HashSet::new()).insert(x);
    }

    let mut ans = usize::MAX;
    'outer: for i in (0..N).permutations(N) {
        // 噂を順守するか?
        for (u, v) in i.iter().tuple_windows() {
            if cant.entry(*u).or_default().contains(v) {
                continue 'outer;
            }
        }

        let cost = i.iter().enumerate().map(|(j, &a)| A[a][j]).sum();
        ans = ans.min(cost);
    }

    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
