use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (A, B, C): (usize, usize, usize),
    }

    // 起床時刻を 0時に合わせる
    let A = (A + 24 - C) % 24;
    let B = (B + 24 - C) % 24;
    let C = 0;

    eprintln!("{A} {B} {C}");

    if C <= A && A <= B {
        println!("Yes");
    } else {
        println!("No");
    }
}
