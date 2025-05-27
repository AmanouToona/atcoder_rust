use proconio::input;
#[allow(non_snake_case)]

struct Solution {
    H: usize,
    W: usize,
    A: Vec<Vec<usize>>,
    used: Vec<Vec<bool>>,
    res: usize,
}

#[allow(non_snake_case)]

impl Solution {
    fn new(H: usize, W: usize, A: Vec<Vec<usize>>) -> Self {
        Self {
            H,
            W,
            A,
            used: vec![vec![false; W]; H],
            res: 0,
        }
    }

    fn calc_score(&self) -> usize {
        let mut score = 0;
        for h in 0..self.H {
            for w in 0..self.W {
                if self.used[h][w] {
                    continue;
                }
                score ^= self.A[h][w];
            }
        }
        score
    }

    fn dfs(&mut self, h: usize, w: usize) {
        if h == self.H - 1 && w == self.W - 1 {
            let score = self.calc_score();
            if score > self.res {
                self.res = score;
            }
            return;
        }

        // ドミノを置いて次へ
        if !self.used[h][w] {
            if w + 1 < self.W && !self.used[h][w + 1] {
                self.used[h][w] = true;
                self.used[h][w + 1] = true;
                self.dfs(h, w + 1);
                self.used[h][w] = false;
                self.used[h][w + 1] = false;
            }
            if h + 1 < self.H && !self.used[h + 1][w] {
                self.used[h][w] = true;
                self.used[h + 1][w] = true;

                if w + 1 < self.W {
                    self.dfs(h, w + 1);
                } else {
                    self.dfs(h + 1, 0);
                }
                self.used[h][w] = false;
                self.used[h + 1][w] = false;
            }
        }

        // 何もせずに次へ
        if w + 1 < self.W {
            self.dfs(h, w + 1);
        } else {
            self.dfs(h + 1, 0);
        }
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        A: [[usize; W]; H],
    }

    let mut solution = Solution::new(H, W, A);
    solution.dfs(0, 0);
    println!("{}", solution.res);
}
