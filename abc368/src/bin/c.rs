use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        H: [usize; N],
    }

    let mut T = 0;

    for &h in H.iter() {
        let mut ok = 10usize.pow(10u32);
        let mut ng = 0;

        while ok - ng > 1 {
            let mid = (ok + ng) / 2;

            let three = (mid + T % 3) / 3;

            if three * 3 + (mid - three) >= h {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        T += ok;
    }

    println!("{T}");
}
