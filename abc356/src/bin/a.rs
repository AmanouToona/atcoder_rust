use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, L, R): (usize, usize, usize),
    }

    let mut ans: Vec<usize> = (1..=N).collect();

    for (pos, num) in (L..=R).zip((L..=R).rev()) {
        ans[pos - 1] = num;
    }

    let ans = ans.iter().join(" ");

    println!("{}", ans);
}
