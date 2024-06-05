use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (a, b, c): (u64, u32, u64),
    }

    if a < c.pow(b) {
        println!("Yes");
    } else {
        println!("No");
    }
}
