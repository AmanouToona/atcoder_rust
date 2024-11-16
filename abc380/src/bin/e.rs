use either::Either::Right;
use im_rc::HashMap;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }

    let mut color: Vec<usize> = (0..N).collect();
    let mut left = (0..N).collect();
    let mut right = (0..N).collect();

    let mut color_cnt: HashMap<usize, usize> = HashMap::new();

    for _ in 0..Q {
        input! {q: usize}

        if q == 1 {
            input! {(x, c): (usize, usize)}

            let x = x - 1;
            let c = c - 1;
        } else if q == 2 {
            input! {c: usize}
        } else {
            println!("strange input !!!");
        }
    }
}
