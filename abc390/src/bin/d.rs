use proconio::input;
use std::collections::HashSet;

struct Solve {
    n: usize,
    a: Vec<usize>,
    b: Vec<Vec<usize>>,
    xor: HashSet<usize>,
}

impl Solve {
    fn new(n: usize, a: Vec<usize>) -> Self {
        Solve {
            n,
            a,
            b: vec![],
            xor: HashSet::new(),
        }
    }

    fn dfs(&mut self, i: usize) {
        if i == self.n {
            let mut xor = 0;
            for bs in self.b.iter() {
                xor ^= bs.iter().sum::<usize>();
            }
            self.xor.insert(xor);

            return;
        }

        for j in 0..self.b.len() {
            self.b[j].push(self.a[i]);
            self.dfs(i + 1);
            self.b[j].pop();
        }

        self.b.push(vec![self.a[i]]);
        self.dfs(i + 1);
        self.b.pop();
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut solve = Solve::new(N, A);

    solve.dfs(0);

    println!("{}", solve.xor.len());
}
