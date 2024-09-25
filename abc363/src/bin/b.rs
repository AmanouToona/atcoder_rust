use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, T, P): (usize, usize, usize),
        mut L: [usize; N],
    }
    L.sort();
    L = L.into_iter().rev().collect();

    let ans = T.checked_sub(L[P - 1]).unwrap_or(0);

    println!("{ans}");
}
