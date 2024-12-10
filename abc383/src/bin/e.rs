use ac_library::Dsu;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M, K): (usize, usize, usize),
        mut uvw : [(usize, usize, usize); M],
        A: [usize; K],
        B: [usize; K],
    };

    // A, B 2色分け. Aを正, Bを負で塗る
    let mut color: Vec<i64> = vec![0; N];

    for &a in A.iter() {
        color[a - 1] += 1;
    }
    for &b in B.iter() {
        color[b - 1] -= 1;
    }

    // 小さな辺から採用する
    uvw.sort_by_key(|x| x.2);

    let mut uf = Dsu::new(N);
    let mut ans = 0;
    for &(u, v, w) in uvw.iter() {
        let u = u - 1;
        let v = v - 1;

        if uf.same(u, v) {
            continue;
        }

        let u = uf.leader(u);
        let v = uf.leader(v);

        // 違う色の頂点を最小重み経路で結合する
        if color[u] * color[v] < 0 {
            ans += w * ((color[u].abs().min(color[v].abs())) as usize);
        }

        let res = color[u] + color[v];

        uf.merge(u, v);

        let u = uf.leader(u);
        color[u] = res;
    }

    println!("{ans}");
}
