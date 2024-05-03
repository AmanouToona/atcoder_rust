use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        CP: [(usize, usize); N],
        Q: usize,
        LR: [(usize, usize); Q],
    }

    let mut c1 = vec![0; N + 1];
    let mut c2 = vec![0; N + 1];

    for (i, &(c, p)) in CP.iter().enumerate() {
        let i = i + 1;
        if c == 1 {
            c1[i] = p;
        } else if c == 2 {
            c2[i] = p;
        } else {
            panic!("wrong class passed !!");
        }
    }

    // 累積和
    for i in 0..N {
        c1[i + 1] += c1[i];
        c2[i + 1] += c2[i];
    }

    // 回答
    for &(L, R) in LR.iter() {
        let ans1 = c1[R] - c1[L - 1];
        let ans2 = c2[R] - c2[L - 1];

        println!("{} {}", ans1, ans2);
    }
}
