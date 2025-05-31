use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, u32),
        A: [i128; N]
    }

    let mut ans = 1;

    for &a in A.iter() {
        ans *= a;

        if ans >= 10i128.pow(K) {
            ans = 1;
        }
    }
    println!("{ans}");
}
