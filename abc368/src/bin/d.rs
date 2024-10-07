use im_rc::HashSet;
use proconio::input;
use std::collections::BinaryHeap;
#[allow(non_snake_case)]
struct Search {
    node2num: Vec<usize>,
    num2node: Vec<usize>,
    fp: Vec<bool>,
    n: usize,
}

impl Search {
    fn new(N: usize) -> Self {
        Search {
            node2num: vec![0; N],
            num2node: vec![0; N],
            fp: vec![false; N],
            n: 0,
        }
    }
}

fn dfs(u: usize, g: &Vec<Vec<usize>>, s: &mut Search) {
    s.fp[u] = true;
    s.node2num[u] = s.n;
    s.num2node[s.n] = u;
    s.n += 1;

    for &v in g[u].iter() {
        if s.fp[v] == true {
            continue;
        }
        dfs(v, &g, s);
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        (N, K): (usize, usize),
        AB: [(usize, usize); N - 1],
        V: [usize; K],
    }

    let mut g = vec![Vec::new(); N];
    for &(a, b) in AB.iter() {
        let a = a - 1;
        let b = b - 1;
        g[a].push(b);
        g[b].push(a);
    }

    let V: Vec<usize> = V.into_iter().map(|x| x - 1).collect();

    let mut node2num = vec![0; N];
    let mut num2node = vec![0; N];
    let mut fp = vec![false; N];

    let mut search = Search::new(N);

    dfs(0, &g, &mut search);

    let mut ans = 0;
    let mut vs: BinaryHeap<usize> = BinaryHeap::new();

    for &n in V.iter() {
        vs.push(search.node2num[n]);
    }

    let mut done = HashSet::new();

    while vs.len() > 1 {
        let u = vs.pop().unwrap();
        let u = search.num2node[u];

        for &v in g[u].iter() {
            if search.node2num[v] > search.node2num[u] {
                continue;
            }

            ans += 1;
            let v_nom = search.node2num[v];
            if done.contains(&v_nom) {
                break;
            }

            vs.push(v_nom);
            done.insert(v_nom);
        }
    }

    println!("{}", ans + 1);
}
