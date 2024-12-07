use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
    }

    let mut left = vec![0; 26];
    let mut right = vec![0; 26];

    for &s in S.iter() {
        let num = s as usize - 'A' as usize;
        right[num] += 1;
    }

    let mut ans: usize = 0;

    for &s in S.iter() {
        let num = s as usize - 'A' as usize;
        right[num] -= 1;
        for j in 0..26 {
            ans += left[j] * right[j];
        }

        left[num] += 1;
    }

    println!("{ans}");
}
