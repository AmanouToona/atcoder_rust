use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: Chars,
    }

    let S: Vec<usize> = S.into_iter().map(|x| x as usize - '0' as usize).collect();

    let mut cumsum = Vec::new();
    for (i, &s) in S.iter().enumerate() {
        cumsum.push((i + 1) * s);
    }
    for i in 1..N {
        cumsum[i] += cumsum[i - 1];
    }

    let mut ans = Vec::new();

    let mut sum = 0;
    for i in (0..N).rev() {
        sum += cumsum[i];
        ans.push(sum % 10);
        sum /= 10;
    }

    while sum > 0 {
        ans.push(sum % 10);
        sum /= 10;
    }

    let ans: String = ans
        .iter()
        .rev()
        .map(|&x| ((x + '0' as usize) as u8) as char)
        .collect();
    println!("{ans}");
}
