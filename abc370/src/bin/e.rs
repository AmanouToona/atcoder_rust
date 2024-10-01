use ac_library::ModInt998244353 as Mint;
use proconio::input;
#[allow(non_snake_case)]
struct DP {
    dp: Vec<Vec<Option<Mint>>>, // 半開区間 dp[left][right] 条件を満たす
    cumsum: Vec<i64>,           // 半開区間
    K: i64,
    N: usize,
}
#[allow(non_snake_case)]
impl DP {
    fn new(N: usize, K: i64, A: &Vec<i64>) -> Self {
        let mut cumsum = vec![0];

        for (i, &a) in A.iter().enumerate() {
            cumsum.push(cumsum[i] + a);
        }

        let mut dp = vec![vec![None; N + 1]; N + 1];

        for i in 0..N + 1 {
            dp[i][i] = Some(Mint::new(0));
        }

        for i in 0..N {
            dp[i][i + 1] = Some(Mint::new(0));
        }

        for (i, a) in A.iter().enumerate() {
            if *a != K {
                if let Some(ref mut v) = dp[i][i + 1] {
                    *v += 1;
                }
            }
        }

        DP { dp, cumsum, K, N }
    }

    fn rec(&mut self, left: usize, right: usize) -> Mint {
        if let Some(v) = self.dp[left][right] {
            return v;
        }

        let mut cnt = Mint::new(0);

        for k in left + 1..right {
            if self.cumsum[k] - self.cumsum[left] != self.K {
                cnt += self.rec(k, right);
            }
        }

        if self.cumsum[right] - self.cumsum[left] != self.K {
            cnt += 1;
        }

        self.dp[left][right] = Some(cnt);
        cnt
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, i64),
        A: [i64; N],
    }

    let mut solve = DP::new(N, K, &A);
    let ans = solve.rec(0, N);

    println!("{}", ans);
}
