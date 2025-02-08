use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        uv: [(usize, usize); N - 1],
    }

    let uv: Vec<(usize, usize)> = uv.into_iter().map(|(u, v)| (u - 1, v - 1)).collect();

    let mut g = vec![Vec::new(); N];

    for &(u, v) in uv.iter() {
        g[u].push(v);
        g[v].push(u);
    }

    let mut ans = N;
    for x in 0..N {
        let mut degs = Vec::new();
        for &y in &g[x] {
            degs.push(g[y].len() - 1);
        }
        degs.sort_by(|x, &y| y.cmp(x));

        for (i, &d) in degs.iter().enumerate() {
            ans = ans.min(N - ((d + 1) * (i + 1) + 1));
        }
    }
    println!("{ans}");
}
