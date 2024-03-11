use proconio::input;
#[allow(non_snake_case)]

fn solve(case: i64) -> i64 {
    let dig = digit(case);

    let mut dp = vec![vec![vec![0i64; 2]; 2]; dig as usize + 1];
    dp[0][0][0] = 1;

    for d in 0..dig {
        for ove in 0..=1 {
            for no_neq in 0..=1 {
                for n in 0..=9 {}
            }
        }
    }

    0
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
