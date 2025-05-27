use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        Y : usize
    }

    let mut ok = 0.;
    for i in 1..=6 {
        for j in 1..=6 {
            if i + j >= X || i.abs_diff(j) >= Y {
                ok += 1.;
            }
        }
    }

    let ans = ok / 36.0;
    println!("{}", ans);
}
