use ac_library::SccGraph;
use im_rc::{HashMap, HashSet};
use proconio::input;

struct Solve {
    g: Vec<HashSet<usize>>,
    gs: Vec<usize>,
    n: usize,
}

impl Solve {
    fn new(g: Vec<HashSet<usize>>, gs: Vec<usize>) -> Self {
        Solve { g, gs, n: 0 }
    }

    fn dfs(&mut self, u: usize, nc: usize) {
        for v in self.g[u].iter() {
            let add = self.gs[u];
            let v = *v;
            self.dfs(v, nc + add);
        }
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }

    let mut g = SccGraph::new(N);
    for (i, &a) in A.iter().enumerate() {
        g.add_edge(i, a - 1);
    }

    let group = g.scc();

    let mut n2group = HashMap::new();
    for (i, g_) in group.iter().enumerate() {
        for g__ in g_.iter() {
            n2group.insert(g__, i);
        }
        println!("{:?}", g_);
    }

    let mut g = vec![HashSet::new(); group.len()];
    for g_ in group.iter() {
        for g2 in g_.iter() {
            let u = n2group.get(g2).unwrap();
            let v = n2group.get(&(A[*g2] - 1)).unwrap();
            if u == v {
                continue;
            }
            g[*u].insert(v);
        }
    }

    let g_cnt: Vec<usize> = group.iter().map(|x| x.len()).collect();

    // let mut g = vec![Vec::new(); N];
    // for (i, &a) in A.iter().enumerate() {
    //     g[i].push(a - 1);
    // }
}
