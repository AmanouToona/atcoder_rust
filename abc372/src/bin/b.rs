use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        mut M: i64,
    }

    let mut ans = Vec::new();

    while M > 0 {
        let mut i = 0;

        while i + 1 <= 10 && 3_i32.pow(i + 1) as i64 <= M {
            i += 1;
        }

        ans.push(i);
        M -= 3_i32.pow(i) as i64;
    }

    println!("{}", ans.len());
    let mut ans: String = ans.iter().join(" ");

    println!("{ans}");
}
