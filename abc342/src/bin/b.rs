use im_rc::HashMap;
use proconio::input;
fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: usize,
        ab: [(usize, usize); q]
    }

    let mut position: HashMap<usize, usize> = HashMap::new();

    for (i, p_) in p.into_iter().enumerate() {
        *position.entry(p_).or_default() = i;
    }

    for (a, b) in ab.into_iter() {
        if position.get(&a).unwrap() < position.get(&b).unwrap() {
            println!("{}", a);
        } else {
            println!("{}", b);
        }
    }
}
