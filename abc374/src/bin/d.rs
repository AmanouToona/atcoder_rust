use itertools::Itertools;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, S, T): (usize, f64, f64),
        abcd: [(f64,f64, f64, f64); N],
    }

    let mut per = Vec::new();
    for i in 0..N {
        per.push(i);
    }

    let mut ans = 10f64.powf(15.);
    for p in per.iter().permutations(N) {
        // 向き
        for direction in 0..(1 << 6) {
            let mut time = 0.;
            let mut ux = 0.;
            let mut uy = 0.;

            for i in 0..N {
                let (a, b, c, d) = abcd[*p[i]];

                if (direction >> i) & 1 == 0 {
                    // 順行
                    time += ((ux - a).powf(2.) + (uy - b).powf(2.)).powf(0.5) / S;
                    time += ((a - c).powf(2.) + (b - d).powf(2.)).powf(0.5) / T;
                    ux = c;
                    uy = d;
                } else {
                    // 逆行
                    time += ((ux - c).powf(2.) + (uy - d).powf(2.)).powf(0.5) / S;
                    time += ((c - a).powf(2.) + (d - b).powf(2.)).powf(0.5) / T;
                    ux = a;
                    uy = b;
                }
            }
            ans = ans.min(time);
        }
    }

    println!("{ans}");
}
