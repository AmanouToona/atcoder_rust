use ac_library::ModInt998244353 as Mint;
use proconio::input;

struct Nck {
    finv: Vec<Mint>,
    fac: Vec<Mint>,
}

impl Nck {
    fn new(n: usize) -> Self {
        let mut finv = vec![Mint::new(1); n + 1];
        let mut fac = vec![Mint::new(1); n + 1];
        let mut inv = vec![Mint::new(1); n + 1];

        for i in 2..=n {
            fac[i] = fac[i - 1] * i;
            inv[i] = -inv[998244353 % i] * (998244353 / i);
            finv[i] = finv[i - 1] * inv[i];
        }

        Nck { finv, fac }
    }

    fn get(&self, n: usize, k: usize) -> Mint {
        if n < k {
            return Mint::new(0);
        }
        self.fac[n] * self.finv[k] * self.finv[n - k]
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
        C: [usize; 26],
    }

    let nck = Nck::new(K);

    // dp[i 種類目までの文字を使用した][j 文字の文字列を作成した] = 組み合わせの数
    let mut dp: Vec<Vec<ac_library::StaticModInt<ac_library::Mod998244353>>> =
        vec![vec![Mint::new(0); K + 1]; 27];
    dp[0][0] = Mint::new(1);

    for i in 0..26 {
        // i + 1 文字種目を用いる組み合わせ数を演算するループ
        for j in 0..=K {
            // j 文字の文字列から遷移するループ
            for k in 0..=C[i] {
                // i + 1 文字種目の文字を k 個新たに用いる
                if j + k > K {
                    break;
                }
                let tmp: ac_library::StaticModInt<ac_library::Mod998244353> = dp[i][j];
                dp[i + 1][j + k] += tmp * nck.get(j + k, k);
            }
        }
    }

    let ans: Mint = dp[26].iter().sum::<Mint>() - Mint::new(1);

    println!("{ans}");
}
