use proconio::input;

struct Solve {
    g: Vec<Vec<usize>>,
    n: usize,
    cnt: usize,
}

impl Solve {
    fn new(g: Vec<Vec<usize>>) -> Self {
        let n = g.len();
        Solve { g, n, cnt: 0 }
    }

    fn dfs(&mut self, u: usize, p: Option<usize>) -> usize {
        let mut res = 0;

        let neighbors = self.g[u].clone();

        for &v in neighbors.iter() {
            if Some(v) == p {
                continue;
            }

            let childs = self.dfs(v, Some(u));

            self.cnt += childs * (self.n - childs);

            res += childs;
        }

        res + 1
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize
    }
    let mut g = vec![vec![]; N];

    for _ in 0..(N - 1) {
        input! {(a, b): (usize, usize)};
        let a = a - 1;
        let b = b - 1;

        g[a].push(b);
        g[b].push(a);
    }

    let mut solve = Solve::new(g);
    solve.dfs(0, None);
    let ans = solve.cnt;
    println!("{}", ans);
}
