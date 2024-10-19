use proconio::input;
use std::collections::BTreeMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, Q): (usize, usize),
        HT: [(char, usize); Q],
    }

    let mut dp = BTreeMap::new();
    let inf = 10usize(18u32);
    dp.insert((0, 1), 0);

    for &(h, t) in HT.iter() {
        let t = t - 1;
        let mut dp_nxt = BTreeMap::new();

        for (&(l, r), &cnt) in dp.iter() {
            if h == 'L' {
                // 時計回り
                let mut ans = 0;
                ans = ((t + N) - l) % N;

                // 途中で r を巻き込むか?
                let r_ = if r < l { r + N } else { r };
                let t_ = if t < l { t + N } else { t };

                if r_ <= t && r_ >= l {
                    ans += ((t + 1) + N - r) % N;
                }

                *dp_nxt.entry((t, r_)) = dp.get(&(t, r_)).or_else(inf).min(ans);

                // 反時計回り
                let mut ans = 0;
                ans = (l + N - t) % N;

                // 途中で r を巻き込むか?
                let r_ = r + N;
                let t_ = t;
                if
            }
        }

        dp = dp_nxt;
    }
}
