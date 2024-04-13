use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
        T: Chars,
    }
    let S: Vec<char> = S
        .into_iter()
        .map(|c| c.to_uppercase().next().unwrap())
        .collect();

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
