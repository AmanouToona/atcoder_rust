use im_rc::HashMap;
use proconio::input;
fn main() {
    input! {
        s: String,
    }

    let s: Vec<char> = s.chars().collect();
    let mut cnt: HashMap<char, usize> = HashMap::new();

    for &s_ in s.iter() {
        *cnt.entry(s_).or_insert(0) += 1;
    }

    for (i, s_) in s.into_iter().enumerate() {
        if cnt.get(&s_).unwrap() == &1 {
            println!("{}", i + 1);
            return;
        }
    }
}
