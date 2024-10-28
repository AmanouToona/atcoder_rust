use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        Y: usize,
    }

    if Y % 4 != 0 {
        println!("365");
    } else if Y % 100 != 0 {
        println!("366");
    } else if Y % 400 != 0 {
        println!("365")
    } else {
        println!("366");
    }
}
