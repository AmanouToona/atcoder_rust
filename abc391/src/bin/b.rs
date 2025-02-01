use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        S: [Chars; N],
        T: [Chars; M],
    };

    for a in 0..N {
        'outer: for b in 0..N {
            for da in 0..M {
                for db in 0..M {
                    if a + da >= N || b + db >= N {
                        continue 'outer;
                    }
                    if S[a + da][b + db] != T[da][db] {
                        continue 'outer;
                    }
                }
            }
            println!("{} {}", a + 1, b + 1);
            return;
        }
    }
}
