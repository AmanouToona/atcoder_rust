use itertools::{iproduct, Itertools};
use proconio::input;

struct Solve {
    state: Vec<Vec<char>>,
}

#[allow(non_snake_case)]
impl Solve {
    fn new(N: usize) -> Self {
        Solve {
            state: vec![
                vec!['.'; 3usize.pow(N.try_into().unwrap())];
                3usize.pow(N.try_into().unwrap())
            ],
        }
    }

    fn dfs(&mut self, K: usize, left: usize, top: usize) {
        // println!("k: {} left:{} top:{}", K, left, top);
        if K == 0 {
            // println!("blacken {} {}", left, top);
            // println!("{:?}", self.state);
            self.state[left][top] = '#';
            return;
        }

        for (l, t) in iproduct!(0..3, 0..3) {
            // println!("{}, {}", l, t);
            if l == 1 && t == 1 {
                continue;
            }

            self.dfs(
                K - 1,
                left + 3usize.pow((K - 1).try_into().unwrap()) * l,
                top + 3usize.pow((K - 1).try_into().unwrap()) * t,
            );
        }
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize
    }

    let mut solve = Solve::new(N);

    solve.dfs(N, 0, 0);
    // println!("{:?}", solve.state);

    for s in solve.state.iter() {
        let a = s.iter().join("");
        println!("{}", a);
    }
}
