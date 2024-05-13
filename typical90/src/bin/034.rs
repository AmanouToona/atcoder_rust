use proconio::input;
use std::collections::HashMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
        A:[usize; N],
    }

    let mut s: HashMap<usize, usize> = HashMap::new();

    let mut max_cnt = 0;
    let mut left = 0;
    for (right, &a) in A.iter().enumerate() {
        *s.entry(a).or_default() += 1;

        while s.len() > K {
            *s.get_mut(&A[left]).unwrap() -= 1;
            if s.get(&A[left]) == Some(&0) {
                s.remove(&A[left]);
            }
            left += 1;
        }

        max_cnt = max_cnt.max(right - left + 1);
    }

    println!("{}", max_cnt);
}
