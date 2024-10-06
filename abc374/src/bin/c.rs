use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: [usize; N],
    }

    let mut ans = K.iter().sum::<usize>();

    for g in 1..(1 << N) {
        let mut a = Vec::new();
        let mut b = Vec::new();

        for i in 0..N {
            if (g >> i) & 1 == 0 {
                a.push(K[i]);
            } else {
                b.push(K[i]);
            }
        }

        if a.len() == 0 || b.len() == 0 {
            continue;
        }

        ans = ans.min(a.iter().sum::<usize>().max(b.iter().sum::<usize>()));
    }

    println!("{ans}");
}
