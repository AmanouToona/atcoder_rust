use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W, D): (usize, usize, usize),
        S: [Chars; H],
    }

    let mut ans = 0;
    for i1 in 0..H {
        for j1 in 0..W {
            for i2 in 0..H {
                for j2 in 0..W {
                    if i1 == i2 && j1 == j2 {
                        continue;
                    }

                    if S[i1][j1] == '#' || S[i2][j2] == '#' {
                        continue;
                    }

                    let mut cnt = 0;
                    for i in 0..H {
                        for j in 0..W {
                            if S[i][j] == '#' {
                                continue;
                            }
                            if i.abs_diff(i1) + j.abs_diff(j1) <= D
                                || i.abs_diff(i2) + j.abs_diff(j2) <= D
                            {
                                cnt += 1;
                            }
                        }
                    }

                    ans = ans.max(cnt)
                }
            }
        }
    }
    println!("{ans}")
}
