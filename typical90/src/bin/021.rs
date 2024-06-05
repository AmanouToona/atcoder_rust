use ac_library::SccGraph;
use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        AB : [(usize, usize); M],
    }

    let mut graph = SccGraph::new(N);

    for (a, b) in AB.into_iter() {
        graph.add_edge(a - 1, b - 1);
    }

    let group = graph.scc();

    let mut ans = 0;
    for g in group.iter() {
        if g.len() <= 1 {
            continue;
        }

        ans += g.len() * (g.len() - 1) / 2;
    }

    println!("{}", ans);
}
