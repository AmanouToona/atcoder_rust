use proconio::input;
use std::mem::swap;
#[allow(non_snake_case)]
fn main() {
    input! {N: usize}
    let mut A: Vec<Vec<usize>> = Vec::new();

    for i in 0..N {
        input! {
            a: [usize;i + 1],
        }
        A.push(a.iter().map(|x| x - 1).collect());
    }

    let mut B = vec![Vec::new(); N];
    for i in 0..N {
        for j in 0..N {
            let a = if i >= j { A[i][j] } else { A[j][i] };
            B[i].push(a);
        }
    }

    let mut a = 0;
    for i in 0..N {
        a = B[a][i];
    }
    println!("{}", a + 1);
}
