use proconio::input;

struct Solve {
    ans: usize,
    g: Vec<Vec<usize>>,
    used: Vec<bool>,
    dig: Vec<usize>,
}

impl Solve {
    fn new(g: Vec<Vec<usize>>, dig: Vec<usize>) -> Self {
        let n = g.len();
        Solve {
            ans: 0,
            g: g.clone(),
            used: vec![false; n],
            dig,
        }
    }

    fn solve(&mut self, u: usize) -> usize {
        self.ans = 0;

        if self.dig[u] != 3 {
            return 0;
        }

        self.dfs(u);

        let ans = if self.ans == 0 {
            0
        } else {
            (self.ans - 1) * self.ans / 2
        };

        ans
    }

    fn dfs(&mut self, u: usize) {
        if self.used[u] == true {
            return;
        }

        if self.dig[u] == 3 {
            self.used[u] = true;
        }

        if self.dig[u] == 2 {
            self.ans += 1;
            return;
        }

        for &v in self.g[u].clone().iter() {
            self.dfs(v);
        }
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        uv: [(usize, usize); N - 1],
    }

    let mut g = vec![vec![]; N];

    for &(u, v) in uv.iter() {
        let u = u - 1;
        let v = v - 1;
        g[u].push(v);
        g[v].push(u);
    }

    let dig: Vec<usize> = g.iter().map(|x| x.len()).collect();

    let mut new_g = vec![vec![]; N];
    for (u, &ref g1) in g.iter().enumerate() {
        if g1.len() != 2 && g1.len() != 3 {
            continue;
        }
        for v in g1.iter() {
            if g[*v].len() != 2 && g[*v].len() != 3 {
                continue;
            }
            new_g[u].push(*v);
        }
    }

    let g = new_g;

    let mut solve = Solve::new(g, dig);

    let mut ans = 0;
    for i in 0..N {
        ans += solve.solve(i);
    }

    println!("{ans}");
}
