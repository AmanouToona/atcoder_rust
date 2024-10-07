use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
            N: usize,
    mut        A:[usize; N],
        }

    let mut ans = 0;
    while A.iter().sum::<usize>() > 1 {
        A.sort_by(|x, y| (*y).cmp(x));
        A[0] -= 1;
        if A[1] < 1 {
            break;
        }
        A[1] -= 1;
        ans += 1;
    }

    println!("{ans}");
}
