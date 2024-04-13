use proconio::input;

struct Solve {
    n: usize,
    ans: Vec<String>,
    s: Vec<char>,
}

impl Solve {
    fn new(n: usize) -> Self {
        Solve {
            n,
            ans: Vec::new(),
            s: Vec::new(),
        }
    }

    fn dfs(&mut self) {
        if self.s.len() == self.n {
            let mut cnt = 0;
            for p in self.s.iter() {
                if *p == '(' {
                    cnt += 1;
                } else {
                    cnt -= 1;
                }
                if cnt < 0 {
                    return;
                }
            }
            if cnt == 0 {
                self.ans.push(self.s.iter().collect());
            }
            return;
        }

        for p in ['(', ')'].iter() {
            self.s.push(*p);
            self.dfs();
            self.s.pop();
        }
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {N: usize}

    let mut solve = Solve::new(N);
    solve.dfs();

    for s in solve.ans {
        println!("{}", s);
    }
}
