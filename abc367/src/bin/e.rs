use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
        X: [usize; N],
        A: [usize; N],
    }

    let X: Vec<usize> = X.into_iter().map(|x| x - 1).collect();

    // dp[2の 0..=60乗回動かした][最初の位置(index)] = 今はどの位置の数字(index)?
    let mut dp = vec![vec![0; N]; 61];

    for (i, &x) in X.iter().enumerate() {
        dp[0][i] = x;
    }

    for i in 1..=60 {
        for j in 0..N {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    // eprintln!("{:?}", dp);

    let mut ans = Vec::new();

    for i in 0..N {
        let mut idx = i;
        for b in 0..=60 {
            if (K >> b) & 1 == 1 {
                idx = dp[b][idx];
            }
        }

        ans.push(A[idx]);
    }

    let ans: String = ans.iter().join(" ");
    println!("{ans}");
}
