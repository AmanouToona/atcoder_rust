use ac_library::{Min, ModInt998244353 as Mint};
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {(N, M): (usize, usize)}

    let mut cnts = [Mint::new(0); 61];

    for i in (0..=60).rev() {
        if N >> i & 1 == 0 {
            continue;
        }

        // println!("under: {}", under);
        let under = ((1 << i) - 1) & N;
        cnts[i] += under + 1;

        if i == 0 {
            continue;
        }
        // println!("{}", i);
        // println!("{}", 1i64 << (i - 1));
        for j in 0..i {
            cnts[j] += 1i64 << (i - 1);
        }
    }

    let mut ans = Mint::new(0);

    for i in 0..=60 {
        if M >> i & 1 == 0 {
            continue;
        }

        ans += cnts[i];
    }

    println!("{}", ans);

    // println!("{:?}", cnts);
}

// 1152921504606846975 1152921504606846975
