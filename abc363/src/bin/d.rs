use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }

    let mut odd = 10;
    let mut even = 10;

    let mut cnt_tot = 0;
    let mut i = 0;

    loop {
        i += 1;
        if cnt_tot + odd >= N {
            let mut ans = Vec::new();

            let n = N - cnt_tot;

            let mut ans = std::char::from_digit(s as u32, 10).unwrap();

            return;
        }
        cnt_tot += odd;

        if cnt_tot + even >= N {
            return;
        }
        cnt_tot += even;

        odd *= 10;
        even *= 10;
    }
}
