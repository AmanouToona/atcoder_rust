use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let MOD = 10usize.pow(8);

    let mut A = A;
    A.sort();
    let A = A;

    let mut C = A.clone();
    for i in 1..N {
        C[i] += C[i - 1];
    }

    let mut ans = 0;
    for (i, &a) in A.iter().take(N - 1).enumerate() {
        ans += C[N - 1] - C[i];
        ans += a * (N - 1 - i);

        // 10 ** 8 を超えた数分だけ 10 ** 8 を引く
        if A[N - 1] + a < MOD {
            continue;
        }

        if A[i + 1] + a >= MOD {
            ans -= MOD * (N - 1 - i);
            continue;
        }

        let mut left = i + 1;
        let mut right = N;

        while right - left > 1 {
            let mid = (right + left) / 2;

            if A[mid] + a < MOD {
                left = mid;
            } else {
                right = mid;
            }
        }

        ans -= MOD * (N - 1 - left);
    }

    println!("{}", ans);
}
