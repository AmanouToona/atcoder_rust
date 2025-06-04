use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: [usize; N],
    }

    let up_down: Vec<i32> = P
        .windows(2)
        .map(|x| if x[0] < x[1] { 1 } else { -1 })
        .collect();

    // run length
    let mut run = vec![];
    for &i in up_down.iter() {
        if let Some((j, ref mut count)) = run.last_mut() {
            if i == *j {
                *count += 1;
                continue;
            }
        }

        run.push((i, 1));
    }

    let mut up_cnt = Vec::new();
    for (sig, cnt) in run.iter() {
        if sig == &1 {
            up_cnt.push(*cnt);
        }
    }
    let mut ans = 0usize;
    for i in up_cnt.windows(2) {
        ans += i[0] * i[1];
    }

    println!("{ans}");
}
