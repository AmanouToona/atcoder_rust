use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut A = A;
    A.sort();
    let A = A;

    let mut ans = 0;

    for i in 0..(N - 1) {
        let mut s = i + 1;

        loop {
            if A[N - 1] / A[i] == A[s] / A[i] {}
        }
    }

    for (i, &a) in A.iter().take(N - 1).enumerate() {
        let mut s = i + 1;

        loop {
            if s >= N {
                break;
            }
            if A[N - 1] / A[i] == A[s] / A[i] {
                ans += (A[N - 1] / A[s]) * ((N - 1) - s + 1);
                break;
            }

            let mut left = s;
            let mut right = N;

            while right - left > 1 {
                let mid = (right + left) / 2;
                if A[mid] / A[s] > A[s + 1] / A[s] {
                    right = mid;
                } else {
                    left = mid;
                }
            }

            ans += A[s + 1] / A[s] * (left - s);

            s = right;
        }
    }

    println!("{}", ans);
}
