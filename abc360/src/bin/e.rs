use ac_library::ModInt998244353 as Mint;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }

    if N == 1 {
        println!("1");
        return;
    }

    let mut pl = Mint::new(1);
    let mut pnl = Mint::new(0);

    for _ in 0..K {
        let pl_nxt = (pl * Mint::new(N * N - 2 * N + 2) + pnl * Mint::new(2)) / Mint::new(N * N);
        let pnl_nxt = (pl * Mint::new(2 * N - 2) + pnl * Mint::new(N * N - 2)) / Mint::new(N * N);

        pl = pl_nxt;
        pnl = pnl_nxt;
    }
    let ans = pl + Mint::new(N - 1) * Mint::new(N + 2) / Mint::new(2) * pnl / Mint::new(N - 1);

    println!("{ans}");
}
