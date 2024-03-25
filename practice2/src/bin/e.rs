use ac_library::mincostflow::MinCostFlowGraph;
use itertools::iproduct;
use proconio::input;
fn main() {
    input! {
        (n, k): (usize, usize),
        a: [[usize; n]; n],
    }

    let mut graph = MinCostFlowGraph::<isize>::new(2 * n + 2 + 1);

    let s = 2 * n;
    let t = 2 * n + 1;

    let mut offset = 0;
    for item in a.iter() {
        for i in item.iter() {
            offset = offset.max(*i);
        }
    }

    let offset = offset as isize;

    for (r, c) in iproduct!(0..n, 0..n) {
        graph.add_edge(r, n + c, 1, -(a[r][c] as isize) + offset);
    }

    for r in 0..n {
        graph.add_edge(s, r, k as isize, 0);
    }
    for c in 0..n {
        graph.add_edge(n + c, t, k as isize, 0);
    }

    graph.add_edge(s, t, 10isize.pow(12), offset + 1);

    graph.flow(s, t, (n * k) as isize);

    let edges = graph.edges();

    let mut used: Vec<(usize, usize)> = Vec::new();

    let mut tot = 0;
    for edge in edges.iter() {
        if (edge.from == s) | (edge.to == t) | (edge.flow == 0) {
            continue;
        }

        used.push((edge.from, edge.to - n));
        tot += edge.cost - offset;
    }

    let mut ans = vec![vec!["."; n]; n];

    for (r, c) in used.into_iter() {
        ans[r][c] = "X";
    }

    println!("{}", -tot);
    for item in ans.into_iter() {
        let a = item.iter().map(|x| x.to_string()).collect::<String>();
        println!("{}", a);
    }
}
