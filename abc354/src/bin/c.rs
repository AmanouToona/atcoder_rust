use im_rc::HashMap;
use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        AC: [(usize, usize); N],
    }

    let mut num: HashMap<(usize, usize), usize> = HashMap::new();
    for (i, (a, c)) in AC.iter().enumerate() {
        num.insert((*a, *c), i + 1);
    }

    let mut AC = AC;
    AC.sort_by(|x, y| y.0.cmp(&x.0));

    let mut has = Vec::new();
    let mut min_a = usize::MAX;
    let mut min_c = usize::MAX;

    for &(a, c) in AC.iter() {
        if a < min_a && c > min_c {
            continue;
        }

        has.push((a, c));
        min_a = min_a.min(a);
        min_c = min_c.min(c);
    }

    let mut ans = Vec::new();

    for &(a, c) in has.iter() {
        ans.push(num.get(&(a, c)).unwrap());
    }

    ans.sort();

    println!("{}", ans.len());
    let ans: String = ans.iter().map(|x| x.to_string()).join(" ");
    println!("{}", ans);
}
