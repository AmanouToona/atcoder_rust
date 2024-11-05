use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        A: [usize; N],
    }

    let A: Vec<usize> = A.into_iter().map(|x| x % M).collect();
}
