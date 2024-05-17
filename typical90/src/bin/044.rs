use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, Q): (usize , usize),
        A: [usize; N],
    }

    let mut A = A;
    let mut left = 0;

    for _ in 0..Q {
        input! {
            (T, x, y): (usize, usize, usize),
        }

        if T == 1 {
            let x = (x - 1 + left) % N;
            let y = (y - 1 + left) % N;

            A.swap(x, y)
        } else if T == 2 {
            left += N;
            left -= 1;
            left %= N;
        } else if T == 3 {
            let x = (x - 1 + left) % N;
            println!("{}", A[x]);
        }
    }
}
