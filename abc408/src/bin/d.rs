use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
    }

    for _ in 0..T {
        input! {
            N: usize,
            S: Chars,
        }

        let mut runl = vec![(S[0], 0)];
        for &s in S.iter() {
            let len = runl.len();
            if s == runl[len - 1].0 {
                runl[len - 1].1 += 1;
            } else {
                runl.push((s, 1));
            }
        }

        let mut zero = vec![0; runl.len() + 1];
        let mut one = vec![0; runl.len() + 1];

        for (i, &(s, n)) in runl.iter().enumerate() {
            if s == '1' {
                zero[i + 1] = n;
            } else {
                one[i + 1] = n;
            }
        }

        for i in 0..runl.len() {
            zero[i + 1] += zero[i];
            one[i + 1] += one[i];
        }

        println!("{:?}", runl);
        println!("{:?}", zero);
        println!("{:?}", one);
    }
}
