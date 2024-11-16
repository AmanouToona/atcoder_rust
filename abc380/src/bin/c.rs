use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
          N: usize,
          K: usize,
    mut      S: Chars,
      }

    let K = K - 1;
    S.push('0');

    let mut ones = Vec::new();

    let mut l = 0;
    for (i, &s) in S.iter().enumerate().skip(1) {
        if s == S[i - 1] {
            continue;
        }

        if S[i - 1] == '1' {
            ones.push((l, i));
        }

        l = i;
    }

    // println!("{:?}", ones);

    let mut ans_ones = Vec::new();
    ans_ones.push((ones[K - 1].0, ones[K - 1].1 + ones[K].1 - ones[K].0));

    for (i, v) in ones.into_iter().enumerate() {
        if i == K || i == K - 1 {
            continue;
        }
        ans_ones.push(v);
    }

    // println!("{:?}", ans_ones);

    let mut ans = vec![0; N];

    for &(s, t) in ans_ones.iter() {
        for i in s..t {
            ans[i] = 1;
        }
    }

    let ans: String = ans.iter().join("");
    println!("{ans}");
}
