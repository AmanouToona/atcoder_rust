use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, Q): (usize, usize),
    }

    let mut pos: Vec<usize> = (0..N).collect();
    let mut cnt = vec![1; N];
    let mut ans = 0;

    for _ in 0..Q {
        input! {q_num: usize}
        if q_num == 1 {
            input! {
                (P, H): (usize, usize),
            }

            let u = pos[P - 1];
            let v = H - 1;

            cnt[u] -= 1;
            if cnt[u] == 1 {
                ans -= 1
            }

            pos[P - 1] = v;
            cnt[v] += 1;
            if cnt[v] == 2 {
                ans += 1;
            }
        } else {
            println!("{ans}");
        }
    }
}
