// ABC176 C
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut max = 0;
    let mut ans = 0;
    for &a in A.iter() {
        if a < max {
            ans += max - a;
            continue;
        }
        max = a;
    }

    println!("{}", ans);
}
