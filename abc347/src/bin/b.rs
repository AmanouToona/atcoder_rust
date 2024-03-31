use im_rc::HashSet;
use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {S: Chars}

    let mut ss: HashSet<String> = HashSet::new();

    for i in 0..S.len() {
        for j in i + 1..S.len() + 1 {
            ss.insert(S[i..j].iter().map(|x| x.to_string()).collect());
        }
    }

    println!("{}", ss.len());
}
