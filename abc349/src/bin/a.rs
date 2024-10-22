use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [i64; N - 1],
    }

    let ans: i64 = 0 - A.iter().sum::<i64>();

    println!("{}", ans);
}
