use im_rc::HashMap;
use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]

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

    let mut is_dup: bool = false;
    for &v in dup.values() {
        if v <= 1 {
            continue;
        }

        ans -= v * (v - 1) / 2;
        is_dup = true;
    }

    if is_dup {
        ans += 1;
    }

    println!("{}", ans);
}
