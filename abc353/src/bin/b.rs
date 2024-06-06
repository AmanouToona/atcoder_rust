use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
        A: [usize; N],
    }

    let mut ans = 0;

    let mut res = K;
    for &a in A.iter() {
        if res >= a {
            res -= a;
        } else {
            ans += 1;
            res = K;
            res -= a;
        }
    }

    if res < K {
        ans += 1;
    }

    println!("{}", ans);
}
