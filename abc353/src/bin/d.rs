use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: i64,
        A: [i64; N],
    }

    let m = 998244353;

    let mut digit = vec![];
    for a in A.iter() {
        let mut d = 1;
        let mut a = *a;
        while a / 10 > 0 {
            a /= 10;
            d += 1;
        }

        digit.push(d);
    }

    let mut cumsum_digit = vec![];
    for d in digit.iter() {
        cumsum_digit.push(10i64.pow(*d));
    }

    for i in 0..(cumsum_digit.len() - 1) {
        cumsum_digit[i + 1] += cumsum_digit[i];
        cumsum_digit[i + 1] %= m;
    }

    let mut ans = 0;
    for (i, a) in A.iter().enumerate() {
        ans += a * ((cumsum_digit.last().unwrap() + m) - cumsum_digit[i]);
        ans %= m;

        ans += a * i as i64;
        ans %= m;
    }

    println!("{}", ans);
}
