use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, Q): (usize, usize),
        XY: [(i64, i64); N],
        qs: [usize; Q],
    }

    let mut t_min = i64::MAX;
    let mut t_max = i64::MIN;
    let mut s_min = i64::MAX;
    let mut s_max = i64::MIN;

    for &(x, y) in XY.iter() {
        let s = x + y;
        let t = -x + y;

        t_min = t_min.min(t);
        t_max = t_max.max(t);
        s_min = s_min.min(s);
        s_max = s_max.max(s);
    }

    for &q in qs.iter() {
        let (x, y) = XY[q - 1];
        let s = x + y;
        let t = -x + y;

        let mut ans = u64::MIN;

        ans = ans.max(s.abs_diff(s_min));
        ans = ans.max(s.abs_diff(s_max));
        ans = ans.max(t.abs_diff(t_min));
        ans = ans.max(t.abs_diff(t_max));
        println!("{}", ans);
    }
}
