use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
        T: Chars,
    }
    let s_string: String = S.into_iter().collect();

    // Convert the string to uppercase
    let uppercase_s = s_string.to_uppercase();

    let S: Vec<char> = uppercase_s.chars().collect();

    let mut num = 0;

    for s in S {
        if s == T[num] {
            num += 1;
        }
        if num == 3 {
            println!("Yes");
            return;
        }
    }

    if num == 2 && T[2] == 'X' {
        println!("Yes");
    } else {
        println!("No")
    }
}
