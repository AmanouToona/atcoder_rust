use proconio::input;
use std::collections::HashMap;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W, N): (usize, usize, usize),
        XY: [(usize, usize); N],
        Q: usize,
    }

    let mut xs = HashMap::new();
    let mut ys = HashMap::new();

    for &(x, y) in XY.iter() {
        let x = x - 1;
        let y = y - 1;

        xs.entry(x).or_insert(HashSet::new()).insert(y);
        ys.entry(y).or_insert(HashSet::new()).insert(x);
    }

    for _ in 0..Q {
        input! {
            axis: usize,
            num: usize,
        }
        let num = num - 1;

        if axis == 1 {
            println!("{}", xs.entry(num).or_default().len());

            for &y in xs.entry(num).or_default().iter() {
                ys.entry(y).or_default().remove(&num);
            }
            xs.remove(&num);
        } else if axis == 2 {
            println!("{}", ys.entry(num).or_default().len());

            for &x in ys.entry(num).or_default().iter() {
                xs.entry(x).or_default().remove(&num);
            }
            ys.remove(&num);
        } else {
            println!("oop");
        }
    }
}
