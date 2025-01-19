use proconio::input;
#[allow(non_snake_case)]

fn solve(n: usize) -> usize {
    let n: Vec<usize> = n
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();

    // dp[digit: 上から何桁目まで見たか? 1-index][top: 最上位の桁の数字][small: True ならば既に条件を満たしている] = 取りえる蛇数の個数
    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; 2]; 10]; n.len() + 1];
    dp[0][0][0] = 1;

    for digit in 0..n.len() {
        for top in 0..=9 {
            for small in 0..=1 {
                for nxt in 0..=9 {
                    // 最初の桁の処理
                    if top == 0 {
                        if digit == 0 {
                            if nxt == n[0] {
                                dp[digit + 1][nxt][0] += dp[digit][top][small];
                            } else if nxt < n[0] {
                                dp[digit + 1][nxt][1] += dp[digit][top][small];
                            } else {
                                continue;
                            }
                        } else {
                            dp[digit + 1][nxt][1] += dp[digit][top][small];
                        }
                    }

                    // 最初の桁以外の処理
                    if small == 0 && nxt > n[digit] {
                        continue;
                    }
                    if nxt >= top {
                        continue;
                    }

                    let s = if small == 1 || nxt < n[digit] { 1 } else { 0 };
                    dp[digit + 1][top][s] += dp[digit][top][small];
                }
            }
        }
    }
    let mut res = 0;
    for num in 1..=9 {
        res += dp[n.len()][num][1];
    }
    res
}

#[allow(non_snake_case)]

fn main() {
    input! {(L, R): (usize, usize)}

    let ans = solve(R + 1) - solve(L);
    println!("{ans}");
}
