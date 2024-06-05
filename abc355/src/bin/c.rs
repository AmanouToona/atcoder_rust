use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        (N, T): (usize, usize),
        A: [usize; T],
    }

    let mut b = vec![vec![usize::MAX; N]; N];

    for (i, &a) in A.iter().enumerate() {
        let a = a - 1;

        let r = a / N;
        let c = a - r * N;

        b[r][c] = i + 1;
    }

    let mut ans = usize::MAX;
    // 行方向のビンゴ
    for b_ in b.iter() {
        ans = ans.min(*b_.iter().max().unwrap());
    }

    // 列方向のビンゴ
    for i in 0..N {
        let mut a_ = 0;
        for b_ in b.iter() {
            a_ = a_.max(b_[i]);
        }
        ans = ans.min(a_);
    }

    // 左斜め
    let mut a_ = 0;
    for (i, b_) in b.iter().enumerate() {
        a_ = a_.max(b_[i]);
    }
    ans = ans.min(a_);

    // 右斜め
    let mut a_ = 0;
    for i in 0..N {
        a_ = a_.max(b[N - i - 1][i]);
    }
    ans = ans.min(a_);

    if ans != usize::MAX {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
