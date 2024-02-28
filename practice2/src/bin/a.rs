use ac_library::Dsu;
use proconio::input;
fn main() {
    input! {
        (n, q): (usize, usize),
        tuv: [(usize, usize, usize); q],
    }

    let mut dsu = Dsu::new(n);

    for &(t, u, v) in tuv.iter() {
        if t == 1 {
            if dsu.same(u, v) {
                println!("1");
            } else {
                println!("0");
            }
        } else {
            dsu.merge(u, v);
        }
    }
}
