use ac_library::ModInt998244353 as Mint;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize
    }

    let mut digit = 0;
    let mut tmp = N;
    while tmp > 0 {
        tmp /= 10;
        digit += 1;
    }

    let n_10 = Mint::new(10usize.pow(digit));

    let ans =
        Mint::new(N) * (n_10.pow(N.try_into().unwrap()) - 1) / Mint::new(10usize.pow(digit) - 1);

    println!("{}", ans);
}
