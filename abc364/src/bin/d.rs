use proconio::input;

fn under_count(t: i64, v: &Vec<i64>) -> usize {
    // t 以下の数がいくつ含まれるかを返す
    if t < v[0] {
        return 0;
    }

    if v[v.len() - 1] <= t {
        return v.len();
    }

    let mut l = 0;
    let mut r = v.len();

    while r - l > 1 {
        let mid = (r + l) / 2;

        if v[mid] <= t {
            l = mid;
        } else {
            r = mid;
        }
    }

    l + 1
}

#[allow(non_snake_case)]
fn main() {
    input! {
        (N, Q): (usize, usize),
       mut  A: [i64; N],
        BK: [(i64, usize); Q],
    }

    A.sort();

    for &(b, k) in BK.iter() {
        let mut ld = 0;
        let mut rd = b.abs_diff(A[0]).max(b.abs_diff(A[A.len() - 1])); // 条件を満たす最小値にしたい

        if under_count(b, &A) - under_count(b - 1, &A) >= k {
            println!("0");
            continue;
        }

        while rd - ld > 1 {
            let mid = (rd + ld) / 2;

            let min = b - mid as i64;
            let max = b + mid as i64;

            let cnt_max = under_count(max, &A);
            let cnt_min = under_count(min - 1, &A);

            let cnt = cnt_max - cnt_min;
            // println!("{cnt_max}, {cnt_min}, {cnt}, {rd} {ld} {mid}");

            if cnt >= k {
                rd = mid;
            } else {
                ld = mid;
            }
        }

        println!("{rd}");
    }
}
