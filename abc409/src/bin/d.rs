use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize
    }

    for _ in 0..T {
        input! {
            N: usize,
            S: Chars,
        }

        let mut l = N;

        for i in 0..N - 1 {
            if S[i] > S[i + 1] {
                l = i;
                break;
            }
        }

        if l == N {
            println!("{}", S.iter().collect::<String>());
            continue;
        }

        let r = (l..N).find(|&i| S[i] > S[l]).unwrap_or(N);

        let ans: String = S[0..l]
            .iter()
            .chain(S[l + 1..r].iter())
            .chain(S[l..l + 1].iter())
            .chain(S[r..].iter())
            .collect();
        println!("{}", ans);
    }
}
