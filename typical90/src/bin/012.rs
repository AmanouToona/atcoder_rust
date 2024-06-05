use ac_library::Dsu;
use im_rc::HashSet;
use proconio::input;

#[allow(non_snake_case)]
fn conv(r: usize, c: usize, w: usize) -> usize {
    c + r * (2 * w + 1)
}

#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W): (usize, usize),
        Q: usize,
    }

    // 各マスからは 4方向に手を生やす
    let mut uf = Dsu::new((2 * H + 1) * (2 * W + 1));
    let mut colored: HashSet<(usize, usize)> = HashSet::new();

    for _ in 0..Q {
        input! {t: usize}
        if t == 1 {
            input! {(r, c): (usize, usize)};
            let r = 2 * r - 1;
            let c = 2 * c - 1;

            colored.insert((r, c));

            uf.merge(conv(r, c, W), conv(r, c + 1, W));
            uf.merge(conv(r, c, W), conv(r - 1, c, W));
            uf.merge(conv(r, c, W), conv(r, c - 1, W));
            uf.merge(conv(r, c, W), conv(r + 1, c, W));
        } else if t == 2 {
            input! {(r1, c1): (usize, usize), (r2, c2): (usize, usize)};
            let x1 = conv(2 * r1 - 1, 2 * c1 - 1, W);
            let x2 = conv(2 * r2 - 1, 2 * c2 - 1, W);

            if uf.same(x1, x2) && colored.contains(&(2 * r1 - 1, 2 * c1 - 1)) {
                println!("Yes");
            } else {
                println!("No");
            }
        } else {
            println!("wrong method t: {} passed", t);
        }
    }
}
