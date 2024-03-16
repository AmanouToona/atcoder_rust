use proconio::input;
fn main() {
    input! {X: i128}

    let ans = if X >= 0 { (X + 9) / 10 } else { X / 10 };

    println!("{}", ans);
}
