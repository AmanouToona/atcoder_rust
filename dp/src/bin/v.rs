use proconio::input;

struct ReRoot {
    g: Vec<Vec<usize>>,
    m: usize,
    dp: Vec<Vec<usize>>,
    deg: Vec<usize>,
}

impl ReRoot {
    fn new(m: usize, g: &Vec<Vec<usize>>) -> Self {
        let n = g.len();
        let deg = g.iter().map(|x| x.len()).collect();

        // dp[頂点番号][is black] = 塗分け方法の個数
        let dp = vec![vec![0; 2]; n];
        ReRoot {
            g: g.clone(),
            m,
            dp,
            deg,
        }
    }

    fn dfs(&mut self, u: usize, p: usize) {
        // 終端処理
        if (self.deg[u] == 1 && self.g[u][0] == p) || self.deg[u] < 1 {
            self.dp[u][0] = 1;
            self.dp[u][1] = 1;
            return;
        }

        let mut black = 1;
        let mut white = 1;
        for v in self.g[u].clone() {
            if v == p {
                continue;
            }

            self.dfs(v, u);
            black *= self.dp[v][1] + self.dp[v][0];
            white *= self.dp[v][0];

            black %= self.m;
            white %= self.m;
        }
        self.dp[u][0] = white;
        self.dp[u][1] = black;
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        XY: [(usize, usize); N - 1],
    }

    let mut g = vec![vec![]; N];
    for (x, y) in XY {
        let x = x - 1;
        let y = y - 1;

        g[x].push(y);
        g[y].push(x);
    }

    for i in 0..N {
        let mut reroot = ReRoot::new(M, &g);
        reroot.dfs(i, usize::MAX);
        println!("{}", reroot.dp[i][1]);
    }
}
