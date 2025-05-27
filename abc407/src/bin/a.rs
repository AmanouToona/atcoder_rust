use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        A: i64, B: i64,
    }

    let n = A / B;
    if 2 * A - 2 * B * n - B <= 0 {
        println!("{n}");
    } else {
        println!("{}", n + 1);
    }
}
