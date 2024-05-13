use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W): (usize, usize),
    }

    if H == 1 || W == 1 {
        println!("{}", H * W);
        return;
    }

    let ans = ((H + 1) / 2) * ((W + 1) / 2);
    println!("{}", ans);
}
