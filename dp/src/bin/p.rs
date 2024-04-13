use ac_library::ModInt1000000007 as Mint;
use proconio::input;

struct Dp {
    g: Vec<Vec<usize>>,
    degree: Vec<usize>,
    dp: Vec<Vec<Mint>>,
}

impl Dp {
    fn new(g: &Vec<Vec<usize>>) -> Self {
        let n = g.len();
        let degree = g.iter().map(|x| x.len()).collect();

        // dp [頂点][is black] = 塗分け方法の個数
        let dp = vec![vec![Mint::new(0); 2]; n];
        Dp {
            g: g.clone(),
            degree,
            dp,
        }
    }

    fn dfs(&mut self, u: usize, p: usize) {
        // 終端条件
        if (self.degree[u] == 1 && self.g[u][0] == p) || self.degree[u] == 0 {
            self.dp[u][0] = Mint::new(1);
            self.dp[u][1] = Mint::new(1);
            return;
        }

        self.dp[u][0] = Mint::new(1);
        self.dp[u][1] = Mint::new(1);

        for v in self.g[u].clone() {
            if v == p {
                continue;
            }

            self.dfs(v, u);

            let temp = self.dp[v][0] + self.dp[v][1];
            self.dp[u][0] *= temp;

            let temp = self.dp[v][0];
            self.dp[u][1] *= temp;
        }
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        XY: [(usize, usize); N - 1]
    }

    let mut g = vec![vec![]; N];

    for (x, y) in XY {
        let x = x - 1;
        let y = y - 1;
        g[x].push(y);
        g[y].push(x);
    }

    let mut dp = Dp::new(&g);
    dp.dfs(0, N + 1);

    let ans = dp.dp[0][0] + dp.dp[0][1];
    println!("{}", ans);
}
