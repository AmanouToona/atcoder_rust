use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut ans = 0;
    for i in 0..=N {
        let mut cnt = 0;

        for &a in A.iter() {
            if a >= i {
                cnt += 1;
            }
        }

        if cnt < i {
            continue;
        }
        ans = i;
    }
    println!("{}", ans);
}
