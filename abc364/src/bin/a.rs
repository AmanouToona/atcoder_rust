use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: [String; N],
    }

    for i in 1..(N - 1) {
        if S[i] == S[i - 1] && S[i] == "sweet" {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
