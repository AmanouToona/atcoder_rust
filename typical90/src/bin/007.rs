use im_rc::HashSet;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        Q: usize,
        B: [usize; Q],
    }

    let mut A: Vec<usize> = A
        .into_iter()
        .collect::<HashSet<usize>>()
        .into_iter()
        .collect();
    A.sort();
    let A = A;

    for b in B {
        if b <= A[0] {
            println!("{}", A[0] - b);
        } else if b >= *A.last().unwrap() {
            println!("{}", b - A.last().unwrap());
        } else {
            let mut min = 0;
            let mut big = A.len() - 1;

            while big - min > 1 {
                let mid = (big + min) / 2;
                if A[mid] >= b {
                    big = mid;
                } else {
                    min = mid;
                }
            }

            let ans = (b - A[min]).min(A[big] - b);
            println!("{}", ans);
        }
    }
}
