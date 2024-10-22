use im_rc::HashMap;
use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
      mut S: Chars,
      mut T: Chars,
    }

    S.push('.');
    S.push('.');

    T.push('.');
    T.push('.');

    let mut cnt = HashMap::new();
    let mut q = VecDeque::new();

    cnt.insert(S.clone(), 0);
    q.push_back(S);

    while let Some(u) = q.pop_front() {
        'outer: for x in 0..(N + 1) {
            if u[x] == '.' || u[x + 1] == '.' {
                continue;
            }

            for k in 0..(N + 1) {
                if u[k] != '.' || u[k + 1] != '.' {
                    continue;
                }

                let mut v = u.clone();
                v[x] = u[k];
                v[x + 1] = u[k + 1];
                v[k] = u[x];
                v[k + 1] = u[x + 1];

                if !cnt.contains_key(&v) {
                    cnt.insert(v.clone(), *cnt.get(&u).unwrap() + 1);
                    q.push_back(v);
                }

                continue 'outer;
            }
        }
    }

    let &ans = cnt.get(&T).unwrap_or(&-1);

    println!("{ans}");
}
