use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        coins: [usize; 3],
    }

    let mut coins = coins;
    coins.sort();
    coins = coins.into_iter().rev().collect();

    let mut ans = usize::MAX;

    for a in (0..=(N / coins[0])).rev() {
        let res = N - a * coins[0];

        for b in (0..=(res / coins[1])).rev() {
            if a + b > 9999 {
                break;
            }

            let res = N - a * coins[0] - b * coins[1];
            if res % coins[2] == 0 {
                let c = res / coins[2];
                ans = ans.min(a + b + c);
            }
        }
    }
    println!("{}", ans);
}
