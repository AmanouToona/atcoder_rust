use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! { S: Chars}

    let mut c_big = 0;

    for s in S.iter() {
        if !s.is_lowercase() {
            c_big += 1;
        }
    }

    if c_big * 2 <= S.len() {
        let ans: String = S.iter().collect();
        let ans = ans.to_lowercase();

        println!("{}", ans);
    } else {
        let ans: String = S.iter().collect();
        let ans = ans.to_uppercase();
        println!("{}", ans);
    }
}
