use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {(N, M): (usize, usize),
    mut B: [i64; N],
    mut W: [i64; M],
        }

    B.sort_by(|x, y| y.cmp(&x));
    W.sort_by(|x, y| y.cmp(&x));

    let mut ans = 0;
    let mut cal = 0;

    // eprintln!("{:?}", B);
    // eprintln!("{:?}", W);

    for (i, &b) in B.iter().enumerate() {
        cal += b;
        // eprintln!("cal {cal}");

        if i < M && W[i] > 0 {
            cal += W[i];
        }

        ans = ans.max(cal);
    }

    println!("{ans}");
}
