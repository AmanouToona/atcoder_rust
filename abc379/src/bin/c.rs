use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        X: [usize; M],
        A: [usize; M],
    }

    let mut stones: Vec<(usize, usize)> =
        X.clone().into_iter().zip(A.clone().into_iter()).collect();

    stones.sort_by_key(|x| x.0);

    if stones.iter().last().unwrap().0 != N {
        stones.push((N, 0));
    }

    if stones[0].0 != 1 {
        println!("-1");
        return;
    }

    let mut u = 1;
    let mut have = stones[0].1;
    let mut ans = 0;

    for &(v, stone) in stones.iter().skip(1) {
        if have < v - u {
            println!("-1");
            return;
        }

        let n = v - u;
        ans += have * n - (n + 1) * n / 2;
        have -= n;

        have += stone;
        u = v;
    }

    if have > 1 {
        println!("-1");
        return;
    }

    println!("{ans}");
}
