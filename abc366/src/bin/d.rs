use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [[i64; N]; N * N],
    }

    let mut cum = vec![vec![vec![0; N + 1]; N + 1]; N + 1];

    for x in 0..N {
        for y in 0..N {
            for z in 0..N {
                cum[x + 1][y + 1][z + 1] = A[x * N + y][z];
            }
        }
    }

    // 3 次元の imos 方法
    for x in 0..N {
        for y in 0..N {
            for z in 0..N {
                cum[x + 1][y + 1][z + 1] -=
                    -cum[x][y + 1][z + 1] - cum[x + 1][y][z + 1] - cum[x + 1][y + 1][z]
                        + cum[x][y][z + 1]
                        + cum[x][y + 1][z]
                        + cum[x + 1][y][z]
                        - cum[x][y][z];
            }
        }
    }

    input! {Q:usize}

    for _ in 0..Q {
        input! {
             (mut lx, rx, mut ly, ry, mut lz, rz): (usize, usize, usize, usize, usize, usize),
        }
        lx -= 1;
        ly -= 1;
        lz -= 1;

        let ans = cum[rx][ry][rz] - cum[lx][ry][rz] - cum[rx][ly][rz] - cum[rx][ry][lz]
            + cum[lx][ly][rz]
            + cum[lx][ry][lz]
            + cum[rx][ly][lz]
            - cum[lx][ly][lz];

        println!("{ans}");
    }
}
