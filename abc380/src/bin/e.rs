use ac_library::Dsu;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }

    let mut cnt = vec![1; N];
    let mut color: Vec<usize> = (0..N).collect();

    // 両閉区間
    let mut left: Vec<usize> = (0..N).collect();
    let mut right: Vec<usize> = (0..N).collect();

    let mut uf = Dsu::new(N);

    for _ in 0..Q {
        input! {q: usize}

        if q == 1 {
            input! {(x, c): (usize, usize)}

            let x = x - 1;
            let c = c - 1;

            let g1 = uf.leader(x);
            // 既に同色なら処理不要
            if color[g1] == c {
                continue;
            }

            // 色の増減処理
            cnt[color[g1]] -= uf.size(g1);
            cnt[c] += uf.size(g1);

            // 左端が左のグループと結合可能か？
            let mut l1 = left[g1];

            if l1 > 0 {
                let r2 = l1 - 1;
                let g2 = uf.leader(r2);
                // 結合可能なら結合する
                if color[g2] == c {
                    l1 = left[g2];
                    uf.merge(g1, g2);
                }
            }

            // 右端が右のグループと結合可能か？
            let mut r1 = right[g1];

            if r1 < N - 1 {
                let l2 = r1 + 1;
                let g2 = uf.leader(l2);

                if color[g2] == c {
                    r1 = right[g2];
                    uf.merge(g1, g2);
                }
            }

            let g1 = uf.leader(x);
            color[g1] = c;
            left[g1] = l1;
            right[g1] = r1;

            // println!("*");
            // println!("{:?}", color);
            // println!("{:?}", left);
            // println!("{:?}", right);
            // println!("*");
        } else if q == 2 {
            input! {c: usize}
            let c = c - 1;
            println!("{}", cnt[c]);
        } else {
            println!("strange input !!!");
        }
    }
}
