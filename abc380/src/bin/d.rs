use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

#[allow(non_snake_case)]
fn solve(S: &Vec<char>, K: usize) -> char {
    let n = S.len();

    let mut double = 0;

    while n * 2usize.pow(double) < K {
        double += 1;
    }

    let mut not = false;
    let mut K = K;
    while double > 0 {
        double -= 1;
        if K > n * 2usize.pow(double) {
            K -= n * 2usize.pow(double);
            not = !not;
        }
    }

    K -= 1;
    let mut ans = S[K];
    if not == true {
        ans = if ans.is_uppercase() {
            ans.to_ascii_lowercase()
        } else {
            ans.to_ascii_uppercase()
        };
    }
    ans
}

#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
        Q: usize,
        K: [usize; Q],
    }

    let S: Vec<char> = S.into_iter().collect();

    let mut ans = Vec::new();
    for &k in K.iter() {
        ans.push(solve(&S, k));
    }

    let ans: String = ans.iter().join(" ");
    println!("{ans}");
}
