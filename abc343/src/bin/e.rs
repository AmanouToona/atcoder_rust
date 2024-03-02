use proconio::input;

fn calc(a2: usize, b2: usize, c2: usize, a3: usize, b3: usize, c3: usize) -> (usize, usize, usize) {
    let mut v1: usize = 0;
    let mut v2: usize = 0;
    let mut v3: usize = 0;

    for a in (0..15).map(|x| x as usize) {
        for b in (0..15).map(|x| x as usize) {
            for c in (0..15).map(|x| x as usize) {
                let mut v = 0;

                if (a <= 6) & (b <= 6) & (c <= 6) {
                    v += 1
                };
                if (a2 <= a) & (a <= a2 + 6) & (b2 <= b) & (b <= b2 + 6) & (c2 <= c) & (c <= c2 + 6)
                {
                    v += 1
                };
                if (a3 <= a) & (a <= a3 + 6) & (b3 <= b) & (b <= b3 + 6) & (c3 <= c) & (c <= c3 + 6)
                {
                    v += 1
                };

                if v == 1 {
                    v1 += 1
                } else if v == 2 {
                    v2 += 1
                } else if v == 3 {
                    v3 += 1
                }
            }
        }
    }
    (v1, v2, v3)
}

fn main() {
    input! {
        (v1, v2, v3): (usize, usize, usize),
    }

    for a2 in 0..=8 {
        for b2 in 0..=8 {
            for c2 in 0..=8 {
                for a3 in 0..=8 {
                    for b3 in 0..=8 {
                        for c3 in 0..=8 {
                            let (v1_, v2_, v3_) = calc(a2, b2, c2, a3, b3, c3);
                            if (v1 == v1_) & (v2 == v2_) & (v3 == v3_) {
                                println!("Yes");
                                println!("0 0 0 {} {} {} {} {} {}", a2, b2, c2, a3, b3, c3);
                                return;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("No");
}
