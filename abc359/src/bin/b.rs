use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; 2 * N],
    }

    let mut pos = vec![Vec::new(); N];

    for (i, &a) in A.iter().enumerate() {
        pos[a - 1].push(i);
    }

    let mut ans = 0;
    for p in pos.iter() {
        if p[0].abs_diff(p[1]) == 2 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
