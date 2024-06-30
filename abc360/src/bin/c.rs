use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        W: [usize; N],
    }

    let mut b = vec![Vec::new(); N];

    for (&a, &w) in A.iter().zip(W.iter()) {
        b[a - 1].push(w);
    }

    let mut ans = 0;
    for p in b.iter() {
        ans += p.iter().sum::<usize>();

        if !p.is_empty() {
            ans -= p.iter().max().unwrap();
        }
    }

    println!("{ans}");
}
