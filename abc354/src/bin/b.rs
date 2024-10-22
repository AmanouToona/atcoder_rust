use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        SC: [(String, usize); N],
    }

    let mut total = 0;
    for (_, c) in SC.iter() {
        total += c;
    }

    let mut S: Vec<String> = SC.iter().map(|(x, _)| x.clone()).collect();
    S.sort();

    let ans = &S[total % N];

    println!("{}", ans);
}
