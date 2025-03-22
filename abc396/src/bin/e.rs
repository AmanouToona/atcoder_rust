use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        XYZ: [(usize, usize, usize); M],
    }

    let mut Z = vec![vec![!0; N]; N];

    for &(x, y, z) in XYZ.iter() {
        let x = x - 1;
        let y = y - 1;

        if Z[x][y] != !0 {
            println!("-1");
            return;
        }

        Z[x][y] = z;
        Z[y][x] = z;
    }

    let mut A = vec![!0; N];
}
