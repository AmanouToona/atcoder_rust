use ac_library::Dsu;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        uv: [(usize, usize); M],
    }

    let mut dsu = Dsu::new(N);

    let mut ans = 0;

    for &(u, v) in uv.iter() {
        let u = u - 1;
        let v = v - 1;

        if dsu.same(u, v) {
            ans += 1;
        } else {
            dsu.merge(u, v);
        }
    }

    println!("{ans}");
}
