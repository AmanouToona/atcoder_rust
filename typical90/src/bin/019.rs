use proconio::input;

#[allow(non_snake_case)]
struct Solve {
    dp: Vec<Vec<usize>>,
    A: Vec<usize>,
}

#[allow(non_snake_case)]
impl Solve {
    fn new(n: usize, A: Vec<usize>) -> Self {
        Solve {
            dp: vec![vec![usize::MAX; 2 * n]; 2 * n],
            A,
        }
    }

    fn get(&mut self, left: usize, right: usize) -> usize {
        if self.dp[left][right] != usize::MAX {
            return self.dp[left][right];
        }

        if right <= left {
            return 0;
        }

        let mut res = usize::MAX;

        for k in ((left + 1)..(right - 1)).step_by(2) {
            let temp = self.get(left, k) + self.get(k + 1, right);
            res = res.min(temp);
        }

        let temp = self.get(left + 1, right - 1) + self.A[left].abs_diff(self.A[right]);
        res = res.min(temp);

        self.dp[left][right] = res;

        self.dp[left][right]
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; 2 * N],
    }

    let mut solve = Solve::new(N, A);

    let ans = solve.get(0, 2 * N - 1);

    println!("{}", ans);
}
