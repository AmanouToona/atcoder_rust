use proconio::input;
#[allow(non_snake_case)]

fn main() {
    input! {X: i128}

    let ans = if X >= 0 { (X + 9) / 10 } else { X / 10 };

    println!("{}", ans);
}
