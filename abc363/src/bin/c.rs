use im_rc::HashSet;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
        S: Chars,
    }

    let candidates: HashSet<Vec<&char>> = S.iter().permutations(N).collect();

    let mut not_ans = 0;
    'outer: for cand in candidates.iter() {
        // 回文であるか確認する
        'inner: for start in 0..=(N - K) {
            for k in 0..K {
                let l = start + k;
                let r = start + K - 1 - k;

                if cand[l] != cand[r] {
                    continue 'inner;
                }
            }
            // 回文あり
            not_ans += 1;
            continue 'outer;
        }
    }

    println!("{}", candidates.len() - not_ans);
}
