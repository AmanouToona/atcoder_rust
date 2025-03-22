use proconio::input;
use std::{collections::HashSet, usize};
#[allow(non_snake_case)]
struct Solve {
    uvw: Vec<Vec<usize>>,
    used: HashSet<usize>,
    ans: usize,
}

impl Solve {
    fn new(N: usize, uvw: &Vec<(usize, usize, usize)>) -> Self {
        let mut res = vec![vec![!0; N]; N];

        for &(u, v, w) in uvw.iter() {
            res[u - 1][v - 1] = w;
            res[v - 1][u - 1] = w
        }

        Solve {
            uvw: res,
            used: HashSet::new(),
            ans: usize::MAX,
        }
    }

    fn solve(&mut self, u: usize, xor: usize) -> usize {
        // println!("{u}, {xor}");

        let n = self.uvw.len();

        if u == n - 1 {
            return xor;
        }

        let mut res = usize::MAX;
        self.used.insert(u);
        for v in 0..n {
            if self.used.contains(&v) {
                continue;
            }
            if self.uvw[u][v] == !0 {
                continue;
            }

            res = res.min(self.solve(v, xor ^ self.uvw[u][v]));
        }
        self.used.remove(&u);

        res
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        uvw: [(usize, usize, usize); M]
    }

    let mut solve = Solve::new(N, &uvw);

    let ans = solve.solve(0, 0);

    println!("{ans}");
}
