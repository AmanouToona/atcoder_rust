use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {(N, S): (usize, usize), T: [usize; N]}

    let mut can = S;
    for &t in T.iter() {
        if t > can {
            println!("No");
            return;
        }
        can = t + S;
    }

    println!("Yes");
}
