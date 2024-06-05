use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
        A: [usize; N],
        B: [usize; N],
    }

    let diff: usize = A.iter().zip(B.iter()).map(|(a, b)| a.abs_diff(*b)).sum();

    if diff > K {
        println!("No");
        return;
    }

    if (K - diff) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
