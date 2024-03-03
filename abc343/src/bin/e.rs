use itertools::iproduct;
use proconio::input;

fn calc(a2: isize, b2: isize, c2: isize, a3: isize, b3: isize, c3: isize) -> (usize, usize, usize) {
    // 3 つの立方体に含まれる領域
    let l_3 = |x1, x2| ((0.min(x1).min(x2) + 7) - 0.max(x1).max(x2)).max(0) as usize;
    let v3 = l_3(a2, a3) * l_3(b2, b3) * l_3(c2, c3);

    // 2 つの立方体にのみ含まれる領域
    let l_2 = |a: isize, b: isize| ((a.min(b) + 7) - a.max(b)).max(0) as usize;
    let a = l_2(0, a2);
    let b = l_2(0, b2);
    let c = l_2(0, c2);

    let mut v2: usize = (a * b * c) as usize;

    let a = l_2(0, a3);
    let b = l_2(0, b3);
    let c = l_2(0, c3);

    v2 += (a * b * c) as usize;

    let a = l_2(a2, a3);
    let b = l_2(b2, b3);
    let c = l_2(c2, c3);

    v2 += (a * b * c) as usize;

    let v2 = v2 - v3 * 3;

    let v1 = 7usize.pow(3) * 3 - v2 * 2 - v3 * 3;

    (v1, v2, v3)
}

fn main() {
    input! {
        (v1, v2, v3): (usize, usize, usize),
    }

    for (a2, b2, c2) in iproduct!(-7..=7, -7..=7, -7..=7) {
        for (a3, b3, c3) in iproduct!(-7..=7, -7..=7, -7..=7) {
            // let (v1_, v2_, v3_) = calculate_volumes(a2, b2, c2, a3, b3, c3);
            let (v1_, v2_, v3_) = calc(a2, b2, c2, a3, b3, c3);

            if (v1 == v1_) & (v2 == v2_) & (v3 == v3_) {
                println!("Yes");
                println!("0 0 0 {} {} {} {} {} {}", a2, b2, c2, a3, b3, c3);
                return;
            }
        }
    }

    println!("No");
}
