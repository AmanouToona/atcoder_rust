// https://atcoder.jp/contests/abc269/tasks/abc269_c
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {N: usize}

    let mut ones = Vec::new();

    for i in 0..60 {
        if 1 << i & N != 0 {
            ones.push(i);
        }
    }

    let mut ans = Vec::new();

    for i in 0..2usize.pow(ones.len() as u32) {
        let mut a = 0i64;

        for (j, one) in ones.iter().enumerate() {
            if i >> j & 1 != 0 {
                a += 1 << one;
            }
        }

        ans.push(a);
    }

    ans.sort();

    for a in ans {
        println!("{}", a);
    }
}
