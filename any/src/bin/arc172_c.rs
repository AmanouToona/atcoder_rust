use proconio::input;
use proconio::marker::Chars;

fn state(n: i32) -> i32 {
    if n < 0 {
        return -1;
    } else if n == 0 {
        return 0;
    } else {
        return 1;
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        C: Chars,
    }

    // 選挙結果を 1: 最初の投票者と同じ, -1: 最初の投票者と異なる で置き換える
    let mut vote = Vec::new();
    for c in C.iter() {
        if c == &C[0] {
            vote.push(1);
        } else {
            vote.push(-1);
        }
    }

    // 選挙結果の累積和
    let mut cum = vote.clone();
    cum[0] = 0;
    for i in 0..vote.len() - 1 {
        cum[i + 1] += cum[i];
    }

    // 投票者1が影響するのは  ai + 1, a(i + 1) + 1 と a(i + 1), a(i + 1) + 1 の状態が異なるとき
    // つまり、 ai + 1 != a(i + 1) のとき
    let mut cnt = 1;
    for i in 0..cum.len() - 1 {
        if state(cum[i] + 1) != state(cum[i + 1]) {
            cnt += 1;
        }
    }

    println!("{cnt}");
}
