use im_rc::HashSet;
use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        A: [usize; N],
        B: [usize; M],
    }

    let setA: HashSet<usize> = A.iter().copied().collect();

    let mut C = A;
    for b in B.iter() {
        C.push(*b);
    }

    C.sort();

    for c in C.iter().tuple_windows::<(&usize, &usize)>() {
        if setA.contains(c.0) && setA.contains(c.1) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
