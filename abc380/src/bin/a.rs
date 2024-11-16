use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: Chars,
    }

    let mut cnt1 = 0;
    let mut cnt2 = 0;
    let mut cnt3 = 0;

    for &n in N.iter() {
        if n == '1' {
            cnt1 += 1;
        } else if n == '2' {
            cnt2 += 1;
        } else if n == '3' {
            cnt3 += 1;
        }
    }

    if cnt1 == 1 && cnt2 == 2 && cnt3 == 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}
