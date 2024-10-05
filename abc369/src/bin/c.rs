use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [i32; N],
    }

    let mut ans = 0;

    let mut R = 0;
    // [L, R)
    for L in 0..N {
        R = (L + 2).min(N).max(R);

        while R < N && A[R] - A[R - 1] == A[R - 1] - A[R - 2] {
            R += 1;
        }

        ans += R - L;
    }

    println!("{ans}");
}
