use ac_library::ModInt998244353;
use proconio::input;

#[allow(non_snake_case)]
fn ways(bit: usize, M: usize) -> ModInt998244353 {
    let mut first_one = 60;

    while first_one > 0 && (M >> first_one) & 1 != 1 {
        first_one -= 1;
    }

    let res: usize = if bit < first_one {
        2usize.pow(bit as u32)
    } else if bit == first_one {
        M as usize - 2usize.pow(first_one as u32) + 1usize
    } else {
        0
    };

    ModInt998244353::new(res)
}

#[allow(non_snake_case)]

fn main() {
    input! {
        N: usize,
        M: usize,
    }

    if N > 60 {
        println!("0");
        return;
    }

    // dp[A 番目][最後尾のAの先頭ビットの位置: 0-index]: 組み合わせ数
    let mut dp = vec![vec![ModInt998244353::new(0); 61]; N + 1];

    for i in 0..=60 {
        dp[1][i] = ways(i, M);
    }

    // 配る dp
    for n in 0..N {
        for u_top in 0..=60 {
            for v_top in (u_top + 1)..=60 {
                let nxt = dp[n][u_top] * ways(v_top, M);
                dp[n + 1][v_top] += nxt;
            }
        }
    }

    let ans = dp[N].iter().sum::<ModInt998244353>();
    println!("{}", ans);
}
