use im_rc::HashSet;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {N: usize, K: usize, A: [usize; N]};

    let mut a_set = HashSet::new();
    for &a in A.iter() {
        if a > K {
            continue;
        }
        a_set.insert(a);
    }

    let A: Vec<usize> = a_set.into_iter().collect();

    let mut ans = (1 + K) * K / 2;

    ans -= A.iter().sum::<usize>();

    println!("{}", ans);
}
