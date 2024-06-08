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

    let mut conect = Mint::new(N);
    let mut ans = Mint::new(0);
    let mut ans_digit = Mint::new(1);

    let mut times = Mint::new(10usize.pow(digit));
    let mut times_digit = Mint::new(digit);

    for i in 0..63 {
        if N >> i & 1 == 1 {
            // ans += conect * 10usize.pow(ans_digit);  みたいなことをしたい
            ans += conect * ans_digit;
            ans_digit *= times;
            // ans_digit *=
        }

        conect += conect * times;
        times_digit *= 2;
        times *= times;
        // println!("{}", conect);
    }

    println!("{}", ans);
}
