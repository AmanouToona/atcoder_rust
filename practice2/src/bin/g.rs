use ac_library::SccGraph;
use proconio::input;
fn main() {
    input! {
        (n, m) : (usize, usize),
        ab: [(usize, usize); m],
    }

    let mut graph = SccGraph::new(n);

    for (a, b) in ab.into_iter() {
        graph.add_edge(a, b);
    }

    let group = graph.scc();

    println!("{}", group.len());
    for g in group.into_iter() {
        let ans: String = g
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{} {}", g.len(), ans);
    }
}
