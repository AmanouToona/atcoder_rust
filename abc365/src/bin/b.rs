use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut A2 = A.clone();
    A2.sort();

    for (i, &a) in A.iter().enumerate() {
        if a == A2[N - 2] {
            println!("{}", i + 1);
        }
    }
}
