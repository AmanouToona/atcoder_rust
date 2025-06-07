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
        let mut r = N;
        let mut p = '=';

        for i in 0..N - 1 {
            if S[i] > S[i + 1] {
                l = i;
                p = S[i];
                while l > 0 && S[l - 1] == S[i] {
                    l -= 1;
                }
                break;
            }
        }

        for i in l..N {
            if S[i] > p {
                r = i;
                break;
            }
        }

        let mut ans = vec!['?'; N];
        for i in 0..N {
            if i >= l && i < r {
                if i != r - 1 {
                    ans[i] = S[i + 1];
                } else {
                    ans[i] = p;
                }
            } else {
                ans[i] = S[i];
            }
        }

        let ans = ans.iter().collect::<String>();
        println!("{}", ans);
        // println!("{} {} {}", l, r, p);
    }
}
