use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        B: [usize; N],
    }
    let mut A = A;
    let mut B = B;

    A.sort();
    B.sort();

    let mut ans = 0;

    for (a, b) in A.iter().zip(B.iter()) {
        ans += a.abs_diff(*b);
    }

    println!("{}", ans);
}
