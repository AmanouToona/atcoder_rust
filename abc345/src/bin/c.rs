use im_rc::HashMap;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        S: Chars,
    }

    let mut dup = HashMap::new();

    for s in S.iter() {
        *dup.entry(s).or_insert(0) += 1;
    }

    let n = S.len();
    let mut ans = n * (n - 1) / 2;

    let mut to_sub = 0;
    for &v in dup.values() {
        if v <= 1 {
            continue;
        }

        ans -= v * (v - 1) / 2 - 1;
        if to_sub == 0 {
            to_sub = 1;
        }
        to_sub *= v;
    }

    ans -= to_sub;

    println!("{}", ans);
}
