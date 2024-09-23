use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        mut M: usize,
    }

    let mut ans = Vec::new();

    while M > 0 {
        let mut a = 0;
        while a + 1 <= 10 && 3i32.pow(a + 1) as usize <= M {
            a += 1;
        }

        ans.push(a);
        M -= 3i32.pow(a) as usize;
    }

    println!("{}", ans.len());
    let ans: String = ans.iter().join(" ");
    println!("{ans}");
}
