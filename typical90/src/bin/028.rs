use im_rc::HashMap;
use itertools::iproduct;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        corner : [(usize, usize, usize, usize); N],
    }

    let mut cum = vec![vec![0i64; 1003]; 1003];

    for &(lx, ly, rx, ry) in corner.iter() {
        let lx = lx + 1;
        let ly = ly + 1;
        let rx = rx;
        let ry = ry;

        cum[lx][ly] += 1;
        cum[rx + 1][ly] -= 1;
        cum[lx][ry + 1] -= 1;
        cum[rx + 1][ry + 1] += 1;
    }

    for (x, y) in iproduct!(0..=1000, 0..=1000) {
        cum[x + 1][y + 1] += cum[x][y + 1] + cum[x + 1][y] - cum[x][y];
    }

    let mut ans = HashMap::new();
    for (x, y) in iproduct!(0..=1001, 0..=1001) {
        *ans.entry(cum[x][y]).or_insert(0) += 1;
    }

    for k in 1..=N {
        let a = ans.entry(k as i64).or_insert(0);
        println!("{}", a);
    }
}

/*
imos 法
非効率
c[i][j + 1] += c[i][j] を i: 0->N, j 0 -> N -1 で計算して、
c[i + 1][j] += c[i][j] を i: 0 -> N -1, j 0-> N で計算。

効率的
c[i + 1][j + 1] = c[i][j + 1] + c[i + 1][j] - c[i][j] + a[i][j]
を i: 0 ->N, j: 0->N で計算。
*/
