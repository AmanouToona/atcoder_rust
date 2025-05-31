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

        // 左から i 個を 1 にするコスト
        let mut l1 = vec![0; N + 1];
        for (i, &s) in S.iter().enumerate() {
            if s == '0' {
                l1[i + 1] += 1;
            }
            l1[i + 1] += l1[i];
        }

        // 左から i 個 0 にするコスト
        let mut l0 = vec![0; N + 1];
        for (i, &s) in S.iter().enumerate() {
            if s == '1' {
                l0[i + 1] += 1;
            }
            if i != 0 {
                l0[i + 1] += l0[i];
            }
        }

        // 右から 0 にするコスト
        let mut r0 = vec![0; N + 1];
        for (i, &s) in S.iter().enumerate().rev() {
            if s == '1' {
                r0[i] += 1;
            }
            r0[i] += r0[i + 1];
        }

        println!("{:?}", l1);
        println!("{:?}", l0);
        println!("{:?}", r0);

        let mut l = 0;
        let mut r = N;
        let mut ans = l1.last().unwrap().clone();

        while l < r {
            let lp = l0[l + 1] + r0[r] + l1[r] - l1[l + 1];
            let rp = l0[l] + r0[r - 1] + l1[r - 1] - l1[l];

            ans = ans.min(lp).min(rp);

            if lp < rp {
                l += 1;
            } else {
                r -= 1;
            }
        }
        println!("{ans}");
    }
}
