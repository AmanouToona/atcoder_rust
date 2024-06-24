use ac_library::SccGraph;
use proconio::input;
use std::collections::{HashMap, HashSet};

struct Solve {
    g: Vec<HashSet<usize>>,
    gs: Vec<usize>,
    ans: usize,
    used: Vec<bool>,
}

impl Solve {
    fn new(g: Vec<HashSet<usize>>, gs: Vec<usize>) -> Self {
        let n = g.len();
        Solve {
            g,
            gs,
            ans: 0,
            used: vec![false; n],
        }
    }

    fn dfs(&mut self, u: usize, cnt: usize) {
        if self.used[u] {
            return;
        }
        self.used[u] = true;

        let nexts = self.g[u].clone();
        for v in nexts {
            if self.used[v] {
                continue;
            }
            self.ans += cnt * self.gs[v];

            self.dfs(u, cnt + self.gs[u]);
        }
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut graph = SccGraph::new(N);
    for (i, &a) in A.iter().enumerate() {
        graph.add_edge(i, a - 1);
    }

    let group = graph.scc();

    let mut node2group = HashMap::new();
    for (i, g) in group.iter().enumerate() {
        for node in g.iter() {
            node2group.insert(*node, i);
        }
    }

    let mut gg = vec![HashSet::new(); group.len()]; // group graph
    for (i, &a) in A.iter().enumerate() {
        let &u = node2group.get(&i).unwrap(); // ここが違う 寝る
        let &v = node2group.get(&(a - 1)).unwrap();
        gg[u].insert(v);
    }

    let mut gs: Vec<usize> = group.iter().map(|x| x.len()).collect(); // group size

    let mut sol = Solve::new(gg, gs);

    for u in 0..group.len() {
        sol.dfs(u, 0);
    }

    println!("{}", sol.ans);
}
