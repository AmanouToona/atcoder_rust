use proconio::input;
use proconio::marker::Chars;
fn conv(x: usize, y: usize, n: usize, i: usize) -> (usize, usize) {
    if i % 4 == 0 {
        return (x, y);
    } else {
        return conv(y.abs_diff(n + 1), x, n, i - 1);
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [Chars; N],
    }

    let mut ans = vec![vec!['.'; N]; N];

    for i in 1..=N {
        for j in 1..=N {
            let cnt = i.min((N + 1).abs_diff(i)).min(j).min((N + 1).abs_diff(j));

            let pos = conv(i, j, N, cnt);
            ans[i - 1][j - 1] = A[pos.0 - 1][pos.1 - 1];
        }
    }

    for a in ans.iter() {
        let a: String = a.iter().collect();
        println!("{a}");
    }
}
