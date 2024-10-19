use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, C): (usize, usize),
        T: [usize; N],
    }

    let mut ans = 1;
    let mut get_time = T[0];

    for &t in T.iter().skip(1) {
        if t - get_time >= C {
            ans += 1;
            get_time = t;
        }
    }

    println!("{ans}");
}
