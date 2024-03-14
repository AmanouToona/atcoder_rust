use nalgebra::abs;
use num::abs_sub;
// https://atcoder.jp/contests/abc119/tasks/abc119_c
use proconio::input;
#[allow(non_snake_case)]

struct Solve {
    L: Vec<usize>,
    A: usize,
    B: usize,
    C: usize,
    ans: usize,
    state: Vec<Vec<usize>>,
}

impl Solve {
    fn dfs(&mut self, i: usize) {
        if i >= self.L.len() {
            for item in self.state.iter() {
                if item.is_empty() {
                    return;
                }
            }

            let mut cost = 0;
            for (i, item) in self.state.iter().enumerate() {
                cost += 10 * (item.len() - 1);

                let length: usize = item.iter().sum();
                if i == 0 {
                    cost += length.abs_diff(self.A);
                } else if i == 1 {
                    cost += length.abs_diff(self.B);
                } else if i == 2 {
                    cost += length.abs_diff(self.C);
                }
            }

            self.ans = self.ans.min(cost);

            return;
        };

        for pos in 0usize..4 {
            if pos == 3 {
                self.dfs(i + 1);
            } else {
                self.state[pos].push(self.L[i]);
                self.dfs(i + 1);
                self.state[pos].pop();
            }
        }
    }
}
#[allow(non_snake_case)]

fn main() {
    input! {
        (N, A, B, C): (usize, usize, usize, usize),
        L: [usize; N],
    }

    let mut sol = Solve {
        L,
        A,
        B,
        C,
        ans: 10000000usize,
        state: vec![vec![], vec![], vec![]],
    };
    sol.dfs(0);

    println!("{}", sol.ans);
}
