use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, R, C): (usize, i64, i64),
        S: Chars,
    }

    let mut R: usize = (R + (N * 2) as i64).try_into().unwrap();
    let mut C: usize = (C + (N * 2) as i64).try_into().unwrap();

    let mut r_s = N * 2;
    let mut c_s = N * 2;

    let mut smoke = HashSet::new();
    smoke.insert((r_s, c_s));

    let mut ans = Vec::new();
    for &s in S.iter() {
        // println!("{R} {C}, {r_s} {c_s}");
        if s == 'N' {
            r_s += 1;
            R += 1;
        } else if s == 'W' {
            c_s += 1;
            C += 1;
        } else if s == 'S' {
            r_s -= 1;
            R -= 1;
        } else if s == 'E' {
            c_s -= 1;
            C -= 1;
        } else {
            println!("ooop");
            return;
        }

        smoke.insert((r_s, c_s));

        // println!("{}", m[R][C]);
        // println!("{}", m[3][7]);
        // println!("{R} {C}\n");

        if smoke.contains(&(R, C)) {
            ans.push("1");
        } else {
            ans.push("0");
        }
    }

    let ans: String = ans.iter().join("");
    println!("{ans}");
}
