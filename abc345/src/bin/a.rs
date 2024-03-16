use proconio::input;
use proconio::marker::Chars;
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

    for i in 1..(S.len() - 1) {
        if S[i] != '=' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
