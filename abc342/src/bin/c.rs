use itertools::Itertools;
use proconio::input;
use std::char::from_u32;
fn main() {
    input! {
        _: usize,
        s: String,
        q: usize,
        cd: [(char, char); q],
    }

    let mut dict: Vec<char> = (0..26).map(|x| from_u32('a' as u32 + x).unwrap()).collect();

    for (c, d) in cd.into_iter() {
        for to in dict.iter_mut() {
            if *to == c {
                *to = d;
            }
        }
    }

    let mut ans = Vec::new();
    for s_ in s.as_str().chars() {
        ans.push(dict[s_ as usize - 'a' as usize]);
    }

    let ans = ans.iter().join("");
    println!("{}", ans);
}
