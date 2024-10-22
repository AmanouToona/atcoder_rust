use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {N: usize, S:[String; N]};

    let mut ans = 0;

    for s in S.iter() {
        if s == "Takahashi" {
            ans += 1;
        }
    }

    println!("{}", ans);
}
