use num_traits::WrappingSub;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        TV: [(usize, usize); N]
    }

    let mut water: usize = 0;
    let mut time = 0;

    for &(t, v) in TV.iter() {
        water = water.saturating_sub(t - time);
        water += v;
        time = t;
    }

    println!("{water}");
}
