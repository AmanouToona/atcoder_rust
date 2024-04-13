use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, L): (usize, usize),
        K: usize,
        mut A: [usize; N],
    }

    A.push(L);
    let A = A;

    let mut ok = 0;
    let mut ng = L + 1;

    while ng - ok > 1 {
        let mid = (ok + ng) / 2;

        let mut cut = 0;
        let mut count = 0;
        for a in A.iter() {
            if a - cut >= mid {
                count += 1;
                cut = *a;
            }
        }

        if count > K {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}
