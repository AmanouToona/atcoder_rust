use proconio::input;
use proconio::marker::Chars;
// ABC326 D

fn is_ok(board: &Vec<Vec<char>>, R: &Vec<char>, C: &Vec<char>) -> bool {
    let mut br = Vec::new();
    for c in board.iter() {
        for m in c.iter() {
            if *m != '.' {
                br.push(*m);
            }
        }
    }

    if br != *R {
        return false;
    }

    let mut bc = Vec::new();
    'outer: for c in 0..board.len() {
        for r in 0..board.len() {
            if board[r][c] != '.' {
                bc.push(board[r][c]);
                continue 'outer;
            }
        }
    }

    if bc != *C {
        return false;
    }

    true
}

struct Solve {
    board: Vec<Vec<char>>,
    R: Vec<char>,
    C: Vec<char>,
    N: usize,
    has: String,
}

impl Solve {
    fn new(R: Vec<char>, C: Vec<char>, N: usize) -> Self {
        let board = vec![vec!['A'; N]; N];
        let has = "No".to_string();

        Solve {
            board,
            R,
            C,
            N,
            has,
        }
    }

    fn dfs(&mut self, n: usize) {
        if n >= self.N {
            if is_ok(&self.board, &self.R, &self.C) {
                self.has = "Yes".to_string();
            }
            return;
        }

        if self.has == "Yes".to_string() {
            return;
        }

        let r = n / self.N;
        let c = n % self.N;

        for ch in ['A', 'B', 'C', '.'] {
            self.board[r][c] = ch;
            self.dfs(n + 1);
        }
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        R: Chars,
        C: Chars,
    }

    let R: Vec<char> = R.into_iter().collect();
    let C: Vec<char> = C.into_iter().collect();

    let mut solve = Solve::new(R, C, N);
    solve.dfs(0);

    if solve.has == "No".to_string() {
        println!("No");
        return;
    }

    for a in solve.board {
        let ans: String = a.iter().collect();
        println!("{}", ans);
    }
}
