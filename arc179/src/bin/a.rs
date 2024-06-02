use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: i64,
        A: [i64; N],
    }

    let mut A = A;
    A.sort();

    let mut q: VecDeque<i64> = VecDeque::from(A);

    let mut ans = Vec::new();
    let mut cumsum = 0i64;

    while !q.is_empty() {
        let u = if cumsum < K {
            q.pop_front().unwrap()
        } else {
            q.pop_back().unwrap()
        };

        if cumsum >= K && cumsum + u < K {
            println!("No");
            return;
        }
        cumsum += u;
        ans.push(u);
    }

    // let ans: String = ans.into_iter().join(" ");
    let ans: String = ans.iter().map(|x| x.to_string()).join(" ");

    println!("Yes");
    println!("{}", ans);
}
