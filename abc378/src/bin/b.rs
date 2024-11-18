use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        qr: [(usize, usize); N],
        Q: usize,
        td: [(usize, usize); Q],
    }

    for &(t, d) in td.iter() {
        let t = t - 1;
        let mut ans = (d / qr[t].0) * qr[t].0 + qr[t].1;

        if ans < d {
            ans += qr[t].0
        }

        println!("{ans}");
    }
}
