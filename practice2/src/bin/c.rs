use ac_library::floor_sum;
use proconio::input;
fn main() {
    input! {
        T: usize,
        nmab: [(i64, i64, i64, i64, ); T],
    }

    for (n, m, a, b) in nmab.into_iter() {
        let ans = floor_sum(n, m, a, b);
        println!("{}", ans);
    }
}
