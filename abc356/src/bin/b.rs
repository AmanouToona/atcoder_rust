use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
                (N, M): (usize, usize),
        A: [usize; M],
    X: [[usize; M]; N],
            }

    let mut tot = vec![0; M];

    for x in X.iter() {
        for (i, x_) in x.iter().enumerate() {
            tot[i] += x_;
        }
    }

    for (a, x) in A.iter().zip(tot.iter()) {
        if x < a {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
