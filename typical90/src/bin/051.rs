use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K, P): (usize, usize, usize),
        A: [usize; N],
    }

    let mut left = vec![Vec::new(); N / 2 + 1];
    for bit in 0..(1 << (N / 2)) {
        let mut price = 0;
        let mut cnt = 0;

        for (i, &a) in A.iter().take(N / 2).enumerate() {
            if bit >> i & 1 == 1 {
                price += a;
                cnt += 1;
            }
        }
        left[cnt].push(price);
    }

    let mut right = vec![Vec::new(); N - N / 2 + 1];
    for bit in 0..(1 << (N - N / 2)) {
        let mut price = 0;
        let mut cnt = 0;

        for (i, &a) in A.iter().skip(N / 2).enumerate() {
            if bit >> i & 1 == 1 {
                price += a;
                cnt += 1;
            }
        }
        right[cnt].push(price);
    }

    let mut ans = 0;

    'outer: for (i, lef) in left.iter().take(K + 1).enumerate() {
        let rig = &mut right[K - i];
        if rig.is_empty() {
            continue;
        }
        rig.sort();
        for l in lef {
            if *l > P {
                continue 'outer;
            }

            let res = P - *l;
            if *rig.last().unwrap() <= res {
                ans += rig.len();
                continue;
            }
            if rig[0] > res {
                continue;
            }

            let mut min = 0;
            let mut big = rig.len();

            while big - min > 1 {
                let mid = (big + min) / 2;
                if rig[mid] > res {
                    big = mid;
                } else {
                    min = mid;
                }
            }

            if *l == 0 && min == 0 {
                continue;
            }

            ans += big;
        }
    }

    println!("{ans}");
}
