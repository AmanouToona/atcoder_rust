use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K):     (usize, usize),
        mut A: [usize; N],
    }

    A.sort();
    let mut ans = usize::MAX;

    for left in 0..K + 1 {
        let right = left + (N - K) - 1;
        ans = ans.min(A[right] - A[left]);
    }

    println!("{ans}");
}
