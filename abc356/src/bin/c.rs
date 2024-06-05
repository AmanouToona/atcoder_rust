use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {(N, M, K): (usize, usize, usize),}

    let mut A = Vec::new();
    let mut R = Vec::new();

    for _ in 0..M {
        input! {
            C: usize,
            a: [usize; C],
            r: char
        }

        let a: Vec<usize> = a.iter().map(|x| x - 1).collect();

        A.push(a);
        R.push(r);
    }

    let mut ans = 0;
    //正しい鍵が 1 の bit 全探索
    for i in 0..(1 << N) {
        let mut ok = true;

        for (a, &r) in A.iter().zip(R.iter()) {
            // 使用した正しい鍵の個数
            let mut cnt = 0;
            for &a_ in a.iter() {
                if (i >> a_) & 1 == 1 {
                    cnt += 1;
                }
            }

            if cnt >= K && r == 'x' {
                ok = false;
            }

            if cnt < K && r == 'o' {
                ok = false
            }
        }

        if ok {
            ans += 1
        }
    }

    println!("{}", ans);
}
