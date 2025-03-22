use proconio::input;
use std::collections::VecDeque;
#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize,
    }

    let mut stack = VecDeque::new();

    for _ in 0..100 {
        stack.push_back(0);
    }

    for _ in 0..Q {
        input! {q: usize}

        if q == 1 {
            input! {x: usize}
            stack.push_back(x);
        } else if q == 2 {
            let ans = stack.pop_back().unwrap();

            println!("{ans}");
        } else {
            println!("oop");
        }
    }
}
