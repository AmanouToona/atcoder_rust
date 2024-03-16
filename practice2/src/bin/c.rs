use ac_library::floor_sum;
use proconio::input;
fn main() {
    input! {
        t: usize,
        nmab: [(i64, i64, i64, i64, ); t],
    }

    for (n, m, a, b) in nmab.into_iter() {
        let ans = floor_sum(n, m, a, b);
        println!("{}", ans);
    }
}
