use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        H: [usize; N],
    }

    let mut ans = 0;
    let mut m = M;
    for &h in H.iter() {
        if m < h {
            break;
        }
        ans += 1;
        m -= h
    }

    println!("{}", ans);
}
