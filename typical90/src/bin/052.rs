use ac_library::ModInt1000000007 as Mint;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N:  usize,
        A: [[usize; 6]; N],
    }

    let mut ans = Mint::new(1);

    for a in A.iter() {
        ans *= a.iter().clone().sum::<usize>();
    }
    println!("{ans}");
}
