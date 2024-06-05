use ac_library::ModInt998244353 as Mint;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {(N, M): (usize, usize)}

    let mut cnts = [Mint::new(0); 61];

    for i in (0..=60).rev() {
        if N >> i & 1 == 0 {
            continue;
        }

        let under = ((1 << i) - 1) & N;
        cnts[i] += under + 1;

        for cnt in cnts.iter_mut().take(i) {
            *cnt += 1i64 << (i - 1);
        }
    }

    let mut ans = Mint::new(0);

    for (i, cnt) in cnts.iter().enumerate() {
        if M >> i & 1 == 0 {
            continue;
        }

        ans += cnt;
    }

    println!("{}", ans);
}
