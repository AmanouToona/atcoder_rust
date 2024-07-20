use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {R: usize}
    let ans = 100 - R % 100;

    println!("{ans}");
}
