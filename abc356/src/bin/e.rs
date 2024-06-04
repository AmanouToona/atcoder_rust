use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let a_max = 10usize.pow(6);

    let mut C = vec![0; a_max + 1];

    for a in A.iter() {
        C[*a] += 1;
    }

    for i in 0..a_max {
        C[i + 1] += C[i];
    }

    let mut ans = 0;

    for h in 1..=a_max {
        let c = C[h] - C[h - 1];
        if c == 0 {
            continue;
        }

        ans += c * (c - 1) / 2;

        if C[h] - C[h - 1] == 0 {
            continue;
        }
        let mut i = 1;
        loop {
            let left = (i * h - 1).max(h);
            if left > a_max {
                break;
            }
            let right = a_max.min((i + 1) * h - 1);

            let cnt = C[h] - C[h - 1];

            ans += (C[right] - C[left]) * i * cnt;

            i += 1;
        }
    }

    println!("{}", ans);
}
