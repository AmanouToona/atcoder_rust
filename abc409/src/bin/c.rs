use proconio::input;
use std::collections::HashMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, L): (usize, i64),
        D: [i64; N -1],
    }

    if L % 3 != 0 {
        println!("0");
        return;
    }

    let mut cnt = HashMap::new();
    cnt.insert(0, 1);

    let mut pos = 0;
    for &d in D.iter() {
        pos += d;
        pos %= L as i64;

        *cnt.entry(pos).or_insert(0) += 1;
    }

    // println!("{:?}", cnt);

    let mut ans: i64 = 0;
    for s in 0..L / 3 {
        let &a = cnt.get(&s).unwrap_or(&0);
        let &b = cnt.get(&(s + L / 3)).unwrap_or(&0);
        let &c = cnt.get(&(s + 2 * L / 3)).unwrap_or(&0);

        ans += a * b * c;
    }

    println!("{}", ans);
}
