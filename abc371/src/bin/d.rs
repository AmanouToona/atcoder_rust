use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: [i64; N],
        P: [usize; N],
        Q: usize,
        LR: [(i64, i64); Q],
    }

    // X が狭義昇順なので座標圧縮は不要
    // 累積和を作っておく
    let mut sum = vec![0; N + 1];

    for i in 1..=N {
        sum[i] += sum[i - 1] + P[i - 1];
    }

    // position x 以下 の座標の街の人口総数を取得する
    let f = |x| -> usize {
        // x 以下の最大座標の町は何番目の街か (1-index)?
        let pos = if x < X[0] {
            0
        } else if x >= X[N - 1] {
            N
        } else {
            let mut min = 1;
            let mut max = N;

            while max - min > 1 {
                let mid = (max + min) / 2;

                if X[mid - 1] > x {
                    max = mid;
                } else {
                    min = mid;
                }
            }
            min
        };

        sum[pos]
    };

    for (l, r) in LR.into_iter() {
        let ans = f(r) - f(l - 1);

        println!("{ans}");
    }
}
