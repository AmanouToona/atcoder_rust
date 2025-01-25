use proconio::input;
use std::collections::HashSet;

struct Sol {
    N: usize,
    A: Vec<i64>,
    B: Vec<i64>,
    XOR: i64,
    X: HashSet<i64>,
}

impl Sol {
    fn new(N: usize, A: Vec<i64>) -> Self {
        let mut xor = 0;
        for a in A.iter() {
            xor ^= a;
        }
        Sol {
            N: N,
            B: A.clone(),
            A: A,
            XOR: xor,
            X: HashSet::new(),
        }
    }

    fn dfs(&mut self, u: usize) {
        if u == self.N - 1 {
            self.X.insert(self.XOR);
            return;
        }

        for i in u..self.N {
            let xor = self.XOR;

            self.XOR ^= self.B[u];
            self.XOR ^= self.B[i];

            self.B[i] += self.A[u];
            self.B[u] -= self.A[u];

            self.XOR ^= self.B[u];
            self.XOR ^= self.B[i];

            self.dfs(u + 1);

            self.B[i] -= self.A[u];
            self.B[u] += self.A[u];

            self.XOR = xor;
        }
        return;
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [i64; N],
    }

    let mut sol = Sol::new(N, A);

    sol.dfs(0);

    println!("{}", sol.X.len());
}
// 12! = 479001600 > 10^8
