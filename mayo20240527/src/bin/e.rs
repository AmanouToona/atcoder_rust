use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
// ABC326 D

#[allow(non_snake_case)]
struct Solve {
    board: Vec<Vec<char>>,
    R: Vec<char>,
    C: Vec<char>,
    n: usize,
}

#[allow(non_snake_case)]
impl Solve {
    fn new(R: Vec<char>, C: Vec<char>) -> Self {
        let n = R.len();
        let board = vec![vec!['.'; n]; n];
        Solve { board, R, C, n }
    }

    fn dfs(&mut self, alp: char) -> bool {
        if alp == 'D' {
            let mut check_R = vec!['.'; self.n];
            'outer: for (i, row) in self.board.iter().enumerate() {
                for &r in row.iter() {
                    if r == '.' {
                        continue;
                    }
                    check_R[i] = r;
                    continue 'outer;
                }
            }

            let mut check_C = vec!['.'; self.n];
            'outer: for j in 0..self.n {
                for row in self.board.iter() {
                    if row[j] == '.' {
                        continue;
                    }
                    check_C[j] = row[j];
                    continue 'outer;
                }
            }

            if self.R == check_R && self.C == check_C {
                return true;
            }
        }

        for position in (0..self.n).permutations(self.n) {
            let mut ok = true;
            for (i, row) in self.board.iter_mut().enumerate() {
                if row[position[i]] != '.' {
                    ok = false;
                    break;
                }
                row[position[i]] = alp;
            }

            if ok && self.dfs(char::from_u32(alp as u32 + 1).unwrap()) {
                return true;
            }

            for (i, row) in self.board.iter_mut().enumerate() {
                if row[position[i]] == alp {
                    row[position[i]] = '.';
                }
            }
        }

        false
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        _: usize,
        R: Chars,
        C: Chars
    }

    let R = R.into_iter().collect();
    let C = C.into_iter().collect();

    let mut solve = Solve::new(R, C);

    if solve.dfs('A') {
        println!("Yes");
        for a in solve.board.iter() {
            let a: String = a.iter().collect();
            println!("{}", a);
        }
    } else {
        println!("No");
    }
}
