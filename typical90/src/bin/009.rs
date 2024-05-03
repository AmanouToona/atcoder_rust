use num_traits::Pow;
use proconio::input;
use std::f64::consts::PI;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        XY: [(f64, f64); N],
    }

    let mut ans: f64 = 0.;

    for (i, &(xi, yi)) in XY.iter().enumerate() {
        let mut thetas: Vec<f64> = Vec::new();

        for (j, &(xj, yj)) in XY.iter().enumerate() {
            if i == j {
                continue;
            }

            let x = xj - xi;
            let y = yj - yi;

            let cos = x / (x.pow(2f64) + y.pow(2f64)).sqrt();
            let mut theta = f64::acos(cos);

            if y < 0. {
                theta *= -1.;
                theta += 2. * PI;
            }

            thetas.push(theta);
            thetas.push(theta + 2. * PI);
        }

        thetas.dedup();
        thetas.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for &a in thetas.iter() {
            if a > 2. * PI {
                break;
            }

            let target = a + PI;

            let mut left = 0;
            let mut right = N;

            while right - left > 1 {
                let mid = (right + left) / 2;

                if thetas[mid] < target {
                    left = mid;
                } else {
                    right = mid;
                }
            }

            let mut candidate = if (target - thetas[left]).abs() < (target - thetas[right]).abs() {
                (a - thetas[left]).abs()
            } else {
                (a - thetas[right]).abs()
            };

            if candidate > PI {
                candidate = 2. * PI - candidate
            }
            ans = ans.max(candidate);
        }
    }

    ans *= 180. / PI;

    println!("{}", ans);
}
