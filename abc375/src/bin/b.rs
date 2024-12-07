use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        XY: [(f64, f64); N],
    }

    let mut xy = vec![(0., 0.)];

    for i in XY.into_iter() {
        xy.push(i);
    }
    xy.push((0., 0.));

    let mut ans = 0.;

    for (i, &(x, y)) in xy.iter().enumerate().skip(1) {
        ans +=
            ((xy[i - 1].0 - x) * (xy[i - 1].0 - x) + (xy[i - 1].1 - y) * (xy[i - 1].1 - y)).sqrt();
    }

    println!("{ans}");
}
