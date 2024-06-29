use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K, P): (usize, usize, usize),
        A: [usize; N],
    }
    let mut H1 = vec![Vec::new(); K + 1]; // half 1
    let mut H2 = vec![Vec::new(); K + 1]; // half 2

    for bit in 0..(1 << (N / 2)) {
        let mut price = 0;
        let mut cnt = 0;

        for (i, &a) in A.iter().take(N / 2).enumerate() {
            if bit >> i & 1 == 1 {
                price += a;
                cnt += 1;
            }
        }
        if cnt > K {
            continue;
        }
        H1[cnt].push(price);
    }

    for bit in 0..(1 << (N - N / 2)) {
        let mut price = 0;
        let mut cnt = 0;

        for (i, &a) in A.iter().skip(N / 2).enumerate() {
            if bit >> i & 1 == 1 {
                price += a;
                cnt += 1;
            }
        }
        if cnt > K {
            continue;
        }
        H2[cnt].push(price);
    }

    // 配列ソート
    for h in H2.iter_mut() {
        h.sort();
    }

    let mut ans = 0;
    for (h1_cnt, h1) in H1.iter().enumerate() {
        let k = K - h1_cnt;
        if H2[k].is_empty() {
            continue;
        }

        for &h in h1.iter() {
            if H2[k][0] + h > P {
                continue;
            }

            if H2[k].last().unwrap() + h <= P {
                ans += H2[k].len();
                continue;
            }

            let mut left = 0;
            let mut right = H2[k].len();

            while right - left > 1 {
                let mid = (right + left) / 2;
                if H2[k][mid] + h > P {
                    right = mid;
                } else {
                    left = mid;
                }
            }

            ans += right;
        }
    }

    println!("{ans}");
}
