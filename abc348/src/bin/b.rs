use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        XY: [(i64, i64); N],
    }

    let mut ans = vec![0; N];

    for (i, &(xi, yi)) in XY.iter().enumerate() {
        let mut dist = 0;
        for (j, &(xj, yj)) in XY.iter().enumerate() {
            if i == j {
                continue;
            }

            let n_dist = xi.abs_diff(xj) * xi.abs_diff(xj) + yi.abs_diff(yj) * yi.abs_diff(yj);

            if n_dist <= dist {
                continue;
            }

            dist = n_dist;
            ans[i] = j;
        }
    }

    for i in ans.iter() {
        println!("{}", i + 1);
    }
}
