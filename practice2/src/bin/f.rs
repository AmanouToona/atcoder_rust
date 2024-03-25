use ac_library::convolution;
use ac_library::ModInt998244353 as Mint;

use proconio::input;
fn main() {
    input! {
        (n,m): (usize,usize),
        a:[Mint;n],
        b:[Mint;m],
    }
    let c = convolution(&a, &b);

    let c: String = c
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", c);
}
