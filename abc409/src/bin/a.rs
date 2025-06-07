use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        T: Chars,
        A:  Chars
    }

    for (&t, &a) in T.iter().zip(A.iter()) {
        if t == a && t == 'o' {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
