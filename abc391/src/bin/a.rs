use std::ops::Index;

use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        D: String,
    }

    let directions = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];

    let mut num = 0;
    for i in 0..directions.len() {
        if directions[i] == D {
            num = i
        }
    }

    println!("{}", directions[(num + 4) % 8]);
}
