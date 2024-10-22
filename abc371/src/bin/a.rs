use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        sab: char,
        sac: char,
        sbc: char,
    }

    let mut cnt = vec![0; 3];

    let ab = if sab == '<' { 1 } else { -1 };
    cnt[0] -= ab;
    cnt[1] += ab;

    let ac = if sac == '<' { 1 } else { -1 };
    cnt[0] -= ac;
    cnt[2] += ac;

    let bc = if sbc == '<' { 1 } else { -1 };
    cnt[1] -= bc;
    cnt[2] += bc;

    for (c, ans) in cnt.iter().zip("ABC".chars()) {
        if *c == 0 {
            println!("{ans}");
        }
    }
}
