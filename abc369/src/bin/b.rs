use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
            N: usize,
    AS: [(usize, char); N],
        }

    let mut ans = 0;
    let mut L: Option<usize> = None;
    let mut R: Option<usize> = None;

    for (a, s) in AS.into_iter() {
        if s == 'L' {
            if let Some(u) = L {
                ans += u.abs_diff(a);
            }
            L = Some(a);
        } else if s == 'R' {
            if let Some(u) = R {
                ans += u.abs_diff(a);
            }
            R = Some(a);
        } else {
            println!("wrong input");
        }
    }
    println!("{ans}");
}
