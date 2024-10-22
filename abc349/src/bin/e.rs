use itertools::iproduct;
use proconio::input;
use std::collections::HashMap;

#[allow(non_snake_case)]
struct Game {
    A: Vec<Vec<isize>>,
    C: Vec<Vec<usize>>, // 0: white, 1: taka, 2: ao,
    cache: HashMap<Vec<Vec<usize>>, bool>,
    taka: bool,
}

#[allow(non_snake_case)]
impl Game {
    fn new(A: Vec<Vec<isize>>) -> Self {
        Game {
            A,
            C: vec![vec![0; 3]; 3],
            cache: HashMap::new(),
            taka: true,
        }
    }

    fn is_end(&self) -> usize {
        // 左上から
        if self.C[0][0] == self.C[1][1] && self.C[1][1] == self.C[2][2] && self.C[2][2] != 0 {
            return self.C[0][0];
        }

        // 右上から
        if self.C[0][2] == self.C[1][1] && self.C[1][1] == self.C[2][0] && self.C[2][0] != 0 {
            return self.C[0][2];
        }

        // 列
        'out: for c in 0..3 {
            for r in 0..2 {
                if self.C[c][r] == 0 {
                    continue 'out;
                }
                if self.C[c][r] != self.C[c][r + 1] {
                    continue 'out;
                }
                if r == 1 {
                    return self.C[c][r + 1];
                }
            }
        }

        // 行
        'out: for r in 0..3 {
            for c in 0..2 {
                if self.C[c][r] == 0 {
                    continue 'out;
                }
                if self.C[c][r] != self.C[c + 1][r] {
                    continue 'out;
                }
                if c == 1 {
                    return self.C[c + 1][r];
                }
            }
        }

        // 全埋め
        for (c, r) in iproduct!(0..3, 0..3) {
            if self.C[c][r] == 0 {
                return 0;
            }
        }

        let mut one = 0;
        let mut two = 0;

        for (c, r) in iproduct!(0..3, 0..3) {
            if self.C[c][r] == 1 {
                one += self.A[c][r];
            } else {
                two += self.A[c][r];
            }
        }

        if one > two {
            return 1;
        }

        2
    }

    // taka が勝つとき true
    fn dfs(&mut self) -> bool {
        if let Some(&t) = self.cache.get(&self.C) {
            return t;
        }

        let end = self.is_end();
        if end != 0 {
            self.cache.entry(self.C.clone()).or_insert(end == 1);
            return end == 1;
        }

        for (c, r) in iproduct!(0..3, 0..3) {
            // すでに塗られている
            if self.C[c][r] != 0 {
                continue;
            }

            // 染色
            if self.taka {
                self.C[c][r] = 1;
            } else {
                self.C[c][r] = 2;
            }
            self.taka = !self.taka;

            let t = self.dfs();

            self.C[c][r] = 0;
            self.taka = !self.taka;

            if self.taka == t {
                self.cache.insert(self.C.clone(), t);
                return t;
            }
        }

        !self.taka
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        A: [[isize; 3]; 3],
    }

    let mut g = Game::new(A);
    if g.dfs() {
        println!("Takahashi")
    } else {
        println!("Aoki")
    }
}
