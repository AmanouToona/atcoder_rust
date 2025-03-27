use ac_library::ModInt998244353 as Mint;
use proconio::input;
use proconio::marker::Chars;
use rand::Rng;
use std::collections::HashSet;

fn make_hash(str: &Vec<char>, base: Mint) -> HashSet<Mint> {
    let mut powers = vec![Mint::new(1)];
    let mut hash: Vec<Mint> = vec![Mint::new(0)];

    for &s in str.iter() {
        let s: u8 = s as u8 - 'A' as u8 + 1;

        hash.push(hash.iter().last().unwrap() * base + Mint::new(s));
        powers.push(powers.iter().last().unwrap() * base);
    }

    let mut suffix = HashSet::new();
    let &h_last = hash.iter().last().unwrap();
    for (i, &h) in hash.iter().enumerate().take(hash.len() - 1) {
        suffix.insert(h_last - h * powers[str.len() - i]);
    }

    suffix
}

#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
    }

    // rolling hash
    let mut rng = rand::thread_rng();
    let power1 = Mint::new(rng.gen_range::<i32, _>(898244350..998244350));
    let power2 = Mint::new(rng.gen_range::<i32, _>(898244350..998244350));
    let power3 = Mint::new(rng.gen_range::<i32, _>(898244350..998244350));

    let suffix1 = make_hash(&S, power1);
    let suffix2 = make_hash(&S, power2);
    let suffix3 = make_hash(&S, power3);

    let mut prefix1 = Mint::new(0);
    let mut prefix2 = Mint::new(0);
    let mut prefix3 = Mint::new(0);

    let mut dup = 0;
    for (i, &s) in S.iter().rev().enumerate() {
        let s: u8 = s as u8 - 'A' as u8 + 1;
        prefix1 = Mint::new(s) + prefix1 * power1;
        prefix2 = Mint::new(s) + prefix2 * power2;
        prefix3 = Mint::new(s) + prefix3 * power3;

        if suffix1.contains(&prefix1) && suffix2.contains(&prefix2) && suffix3.contains(&prefix3) {
            dup = i + 1;
        }
    }

    let mut ans: String = S.iter().take(S.len() - dup).chain(S.iter().rev()).collect();
    println!("{ans}");
}
