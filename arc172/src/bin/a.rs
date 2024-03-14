use proconio::input;
use std::collections::BinaryHeap;
#[allow(non_snake_case)]

fn main() {
    input! {
    (H, W, N): (usize, usize, usize),
    A: [usize; N],
    }
    let mut A = A;

    A.sort();
    let A: Vec<usize> = A.into_iter().rev().map(|x| 2usize.pow(x as u32)).collect();

    let mut piece = BinaryHeap::new();
    piece.push((H.min(W), H, W));

    for &a in A.iter() {
        let p = match piece.pop() {
            Some(p) => p,
            None => {
                println!("No");
                return;
            }
        };

        if p.0 < a {
            println!("No");
            return;
        }

        let x1 = p.1 - a;
        let x2 = p.2 - a;

        if x1 != 0 {
            piece.push((x1.min(a), a, x1));
        }
        if x2 != 0 {
            piece.push((x2.min(a), a, x2));
        }

        if x1 != 0 && x2 != 0 {
            piece.push((x1.min(x2), x1, x2));
        }
    }
    println!("Yes");
}
