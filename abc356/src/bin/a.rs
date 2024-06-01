use itertools::iproduct;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, L, R): (usize, usize, usize),
    }

    let mut ans: Vec<usize> = (1..=N).collect();

    // for (pos, num) in (L..=R).zip(R..L) {
    //     println!("{} {}", pos, num);
    //     ans[pos - 1] = num;
    // }

    for (pos, num) in (L..=R).zip((L..=R).rev()) {
        // println!("{} {}", pos, num);
        ans[pos - 1] = num;
    }

    let ans: Vec<String> = ans.iter().map(|x| x.to_string()).collect();
    let ans = ans.join(" ");

    println!("{}", ans);
}
