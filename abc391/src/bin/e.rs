use proconio::input;
use proconio::marker::Chars;

struct Solve {
    n: usize,
    a: Vec<usize>,
}

impl Solve {
    fn new(n: usize, a: Vec<usize>) -> Self {
        Solve { n, a }
    }

    // (もとの集計値, 変化させる際のコスト)
    fn dfs(&self, left: usize, right: usize) -> (usize, usize) {
        if right == left + 1 {
            return (self.a[left], 1);
        }

        let mut len = right - left;
        let mut costs = vec![Vec::new(); 2];
        for i in 0..3 {
            let (u, c) = self.dfs(left + len / 3 * i, left + len / 3 * (i + 1));
            costs[u].push(c);
        }

        let u = if costs[0].len() > costs[1].len() {
            0
        } else {
            1
        };

        let mut ans = 0;
        let v = 1 - u;
        let mut cos = costs[v].clone();
        cos.sort();

        if cos.len() == 3 {
            ans = cos[0] + cos[1];
        } else {
            ans = cos[0];
        }

        (u, ans)
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: Chars,
    }

    let a: Vec<usize> = A.iter().clone().map(|&x| x as usize).collect();
    println!("{:?}", a);

    let mut sol = Solve::new(N, a);

    let (_, ans) = sol.dfs(0, 3_i32.pow(N.try_into().unwrap()) as usize);
    println!("{ans}");
}
