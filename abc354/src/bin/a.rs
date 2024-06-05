use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        H: i64,
    }

    let mut tall: i64 = 0;
    let mut day: i32 = 0;

    while tall <= H {
        tall += 2i64.pow(day as u32);
        day += 1;
    }

    println!("{}", day);
}
