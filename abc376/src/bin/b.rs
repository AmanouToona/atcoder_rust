use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, Q): (usize, usize),
        HT: [(char, usize); Q],
    }

    let mut ans = 0;
    let mut l = 0;
    let mut r = 1;

    for &(h, t) in HT.iter() {
        let t = t - 1;
        let a = if h == 'L' { l } else { r };
        let b = if h == 'L' { r } else { l };

        let mut ans_ = 10000;
        // 時計回り
        for i in 0..N {
            let a_ = (a + i) % N;
            if a_ == b {
                break;
            }
            if a_ == t {
                ans_ = ans_.min(i);
            }
        }

        // 反時計回り
        for i in 0..N {
            let a_ = (a + N - i) % N;
            if a_ == b {
                break;
            }
            if a_ == t {
                ans_ = ans_.min(i);
            }
        }

        if h == 'L' {
            l = t;
        } else {
            r = t;
        }

        ans += ans_;
    }

    println!("{ans}");
}
