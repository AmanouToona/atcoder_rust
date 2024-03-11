use proconio::input;
#[allow(non_snake_case)]

fn count_eq(case: i64) -> i64 {
    let dig = digit(case);

    // dp[桁数][smaller][non zero][pre num][ok]
    let mut dp = vec![vec![vec![vec![vec![0i64; 2]; 10]; 2]; 2]; dig as usize + 1];
    dp[0][0][0][0][0] = 1;

    let d_nums = nums(case);

    for d in 0..dig {
        for smaller in 0..2 {
            for non_zero in 0..2 {
                for pre in 0..10 {
                    for ok in 0..2 {
                        for n in 0..(if smaller == 1 {
                            10
                        } else {
                            d_nums[d as usize] + 1
                        }) {
                            let mut is_ok = ok;
                            if pre == n {
                                if non_zero == 1 {
                                    is_ok = 1
                                } else if n != 0 {
                                    is_ok = 1;
                                }
                            }

                            dp[d as usize + 1][if (smaller == 1) | (n < d_nums[d as usize]) {
                                1
                            } else {
                                0
                            }][if (non_zero == 1) | (n != 0) { 1 } else { 0 }]
                                [n as usize][is_ok] +=
                                dp[d as usize][smaller][non_zero][pre as usize][ok];
                        }
                    }
                }
            }
        }
    }

    let mut res = 0;
    for i in 0..10 {
        res += dp[dig as usize][1][1][i][1];
        res += dp[dig as usize][1][0][i][1];
    }

    res
}

fn digit(n: i64) -> i64 {
    let mut n = n;
    let mut res = 0;

    while n > 0 {
        res += 1;
        n /= 10;
    }
    res
}

fn nums(n: i64) -> Vec<i64> {
    let mut res = Vec::new();
    let mut n = n;
    while n > 0 {
        res.push(n % 10);
        n /= 10;
    }

    res.reverse();
    res
}

fn solve(case: i64) -> i64 {
    let mut ok = 10i64.pow(15);
    let mut ng = 0i64;

    while ok - ng > 1 {
        let mid = (ok + ng) / 2;

        let n_eq = count_eq(mid);
        let n_neq = mid - n_eq;

        if n_neq >= case {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

fn main() {
    input! {
        T: usize
    }

    for _ in 0..T {
        input! {case: i64}
        let ans = solve(case);
        println!("{}", ans);
    }
}
