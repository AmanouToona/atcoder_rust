use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (A, B): (i32, i32),
    }

    let mut ans = 0;

    for i in -100..=200 {
        let mut num = Vec::from([A, B, i]);
        num.sort();

        if num[2] - num[1] == num[1] - num[0] {
            ans += 1;
        }
    }
    println!("{ans}");
}
