use proconio::input;

const INFINITY: usize = 10usize.pow(10);
#[allow(non_snake_case)]

fn main() {
    input! {
        n: usize,
        A: [usize; n],
        Q: usize,
    }

    let mut query = Vec::new();
    for _ in 0..Q {
        input! {n: usize};
        if n == 1 {
            input! {
                (x, y): (usize, usize),
            }
            query.push((1, x, y));
        } else {
            input! {
                x: usize,
            }
            query.push((2, x, 0));
        }
    }
}
