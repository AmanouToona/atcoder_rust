use proconio::input;

struct Solve {
    g: Vec<Vec<usize>>,
    dp: Vec<usize>, // 利用可能な次数2の頂点
    ans: usize,
}

impl Solve {
    fn new(g: Vec<Vec<usize>>) -> Self {
        let n = g.len();
        Solve {
            g,
            dp: vec![0; n],
            ans: 0,
        }
    }

    fn dfs(&mut self, u: usize, p: Option<usize>) {
        for &v in self.g[u].clone().iter() {
            if !p.is_none() && v == p.unwrap() {
                continue;
            }
            self.dfs(v, Some(u));

            if self.g[u].len() == 3 {
                self.ans += self.dp[u] * self.dp[v];
                self.dp[u] += self.dp[v];
            } else if self.g[u].len() == 2 && self.g[v].len() == 3 {
                self.ans += self.dp[v];
            }
        }

        if self.g[u].len() == 2 {
            self.dp[u] = 1;
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

    let mut solve = Solve::new(g);

    solve.dfs(0, None);
    let ans = solve.ans;

    println!("{ans}");
}
