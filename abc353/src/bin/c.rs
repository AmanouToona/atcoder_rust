use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let m: usize = 10usize.pow(8);

    let mut A = A;
    A.sort();
    let A = A;

    let mut sub = vec![0];
    for (i, &a) in A.iter().enumerate() {
        if i == 0 {
            continue;
        }
        sub.push(A[i] - A[i - 1]);
    }

    for i in 0..(N - 1) {
        sub[i + 1] += sub[i];
    }

    let mut cumsum = A.clone();
    for i in 0..(cumsum.len() - 1) {
        cumsum[i + 1] += cumsum[i];
    }

    let mut ans = 0;
    for (i, a) in A.iter().enumerate() {
        if i == N - 1 {
            break;
        }

        if A[N - 1] + a < m {
            ans += cumsum[N - 1] - cumsum[i];
            ans += a * (N - 1 - i);
            continue;
        }

        if A[i + 1] + a >= m {
            let t = (A[i + 1] + a) / m;

            ans += ()
        }

        // 2分探索で超える位置を特定
        let mut left = i;
        let mut right = N - 1;

        while right - left > 1 {
            let mid = (right + left) / 2;

            if A[mid] + a < m {
                left = mid;
            } else {
                right = mid;
            }
        }

        ans += cumsum[right - 1] - cumsum[i];
        ans += a * (right - i);
    }

    println!("{:?}", sub);
}
