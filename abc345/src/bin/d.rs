use proconio::input;

struct Solve {
    H: usize,
    W: usize,
    N: usize,
    AB: Vec<(usize, usize)>,
    used: Vec<bool>,
    filled: Vec<Vec<bool>>,
    ans: bool,
}

impl Solve {
    // Add a constructor method for `Solve`
    fn new(H: usize, W: usize, N: usize, AB: Vec<(usize, usize)>) -> Self {
        Solve {
            H,
            W,
            N,
            AB,
            used: vec![false; N],
            ans: false,
            filled: vec![vec![false; W]; H],
        }
    }

    fn dfs(&mut self, cnt: usize) {
        // 全タイルを使用した
        if cnt == self.N {
            for r in 0..self.H {
                for c in 0..self.W {
                    if !self.filled[r][c] {
                        return;
                    }
                }
            }
            self.ans = true;
            return;
        }

        let mut r = self.H;
        let mut c = self.W;

        'outer: for _r in 0..self.H {
            for _c in 0..self.W {
                if !self.filled[_r][_c] {
                    r = _r;
                    c = _c;
                    break 'outer;
                }
            }
        }
        // 埋め終わった
        if r == self.H && c == self.W {
            self.ans = true;
            return;
        }

        for i in 0..self.N {
            if self.used[i] {
                continue;
            }

            let mut dr = self.AB[i].0;
            let mut dc = self.AB[i].1;

            for j in 0..2 {
                if j == 1 {
                    std::mem::swap(&mut dr, &mut dc);
                }

                if r + dr > self.H || c + dc > self.W {
                    continue;
                }

                // タイルが他のタイルと重なっていないか確認
                let mut can = true;
                for _r in r..r + dr {
                    for _c in c..c + dc {
                        if self.filled[_r][_c] {
                            can = false;
                            break;
                        }
                    }
                }

                if !can {
                    continue;
                }

                for _r in r..r + dr {
                    for _c in c..c + dc {
                        self.filled[_r][_c] = true;
                    }
                }
                self.used[i] = true;
                self.dfs(cnt + 1);
                self.used[i] = false;
                for _r in r..r + dr {
                    for _c in c..c + dc {
                        self.filled[_r][_c] = false;
                    }
                }
            }
        }
    }
}
fn main() {
    input! {
        (N, H, W): (usize, usize, usize),
        AB: [(usize, usize); N],
    }

    let mut solve = Solve::new(H, W, N, AB);

    solve.dfs(0);
    if solve.ans {
        println!("Yes");
    } else {
        println!("No")
    }
}
