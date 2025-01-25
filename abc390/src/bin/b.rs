use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    if N == 2 {
        println!("Yes");
        return;
    }

    for i in 0..N - 2 {
        if A[i] * A[i + 2] != A[i + 1] * A[i + 1] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
