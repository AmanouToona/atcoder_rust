use amplify::confinement::Collection;
use itertools::Itertools;
use proconio::input;
use std::collections::VecDeque;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut H: [usize; N],
    }

    let mut q = VecDeque::new();

    q.push((usize::MAX, 0));

    let mut time = 0;
    let mut ans = vec![];
    for (i, &h) in H.iter().enumerate() {
        let i = i + 1;
        let mut u_h = 0;
        while !q.is_empty() && q.back().unwrap().0 <= h {
            let (j_h, j) = q.pop_back().unwrap();
            time += (i - j) * (j_h - u_h);
            u_h = j_h;
        }
        let (_, j) = q.back().unwrap();
        time += (i - j) * (h - u_h);
        ans.push(time);
        q.push((h, i));
    }

    let ans: String = ans.iter().map(|x| x + 1).join(" ");
    println!("{}", ans);
}
