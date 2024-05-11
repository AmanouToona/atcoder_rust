use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
            N: usize,
    H: [usize; N],
        }

    for (i, &h) in H.iter().skip(1).enumerate() {
        if h > H[0] {
            println!("{}", i + 2);
            return;
        }
    }

    println!("-1");
}
