use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
        mut B: [usize; N - 1],
    }

    A.sort_by(|x, y| y.cmp(&x));
    B.sort_by(|x, y| y.cmp(&x));

    let inf = 10usize.pow(10u32);
    let mut ans = inf;
    let mut i = 0;
    for &a in A.iter() {
        if i == N - 1 {
            println!("{a}");
            return;
        }
        if a <= B[i] {
            i += 1;
            continue;
        } else if ans != inf {
            println!("-1");
            return;
        } else {
            ans = a;
        }
    }

    println!("{ans}");
}
