use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]

fn main() {
    input! {
        S: Chars,
    }

    if S[0] != '<' {
        println!("No");
        return;
    }
    if *S.last().unwrap() != '>' {
        println!("No");
        return;
    }

    for &s in S.iter().take(S.len() - 1).skip(1) {
        if s != '=' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
