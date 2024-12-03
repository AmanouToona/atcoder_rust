use proconio::input;
use std::collections::VecDeque;
#[allow(non_snake_case)]
fn main() {
    input! {Q: usize}

    let mut day = 0;
    let mut pots = VecDeque::new();
    for _ in 0..Q {
        input! {q: usize}

        match q {
            1 => pots.push_back(day),
            2 => {
                input! {T: usize}
                day += T;
            }
            3 => {
                input! {H: usize}
                let mut cnt = 0;
                while !pots.is_empty() {
                    if day - pots[0] >= H {
                        pots.pop_front();
                        cnt += 1;
                    } else {
                        break;
                    }
                }
                println!("{cnt}");
            }
            _ => println!("wrong query"),
        }
    }
}
