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

        let len = right - left;
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

        let mut cos = costs[u].clone();
        cos.sort();

        let ans = if cos.len() == 3 {
            cos[0] + cos[1]
        } else {
            cos[0]
        };

        (u, ans)
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: Chars,
    }

    let a: Vec<usize> = A
        .iter()
        .clone()
        .map(|&x| x as usize - '0' as usize)
        .collect();

    let sol = Solve::new(N, a);

    let len = 3_i32.pow(N.try_into().unwrap()) as usize;

    let (_, ans) = sol.dfs(0, len);
    println!("{ans}");
}
