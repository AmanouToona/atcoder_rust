use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, P, Q): (usize, usize, usize),
        D: [usize; N],
    }

    let ans = P.min(Q + D.iter().min().unwrap());
    println!("{ans}");
}
