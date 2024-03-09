use ac_library::SccGraph;
use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        (n, d): (usize, isize),
        xy: [(isize, isize); n],
    }

    let mut graph = SccGraph::new(2 * n);

    for i in 0..xy.len() {
        for j in i..xy.len() {
            if i == j {
                continue;
            }

            let (xi, yi) = xy[i];
            let (xj, yj) = xy[j];

            if (xi - xj).abs() < d {
                graph.add_edge(i, j + n);
                graph.add_edge(j, i + n);
            }
            if (xi - yj).abs() < d {
                graph.add_edge(i, j);
                graph.add_edge(j + n, i + n);
            }
            if (yi - xj).abs() < d {
                graph.add_edge(j, i);
                graph.add_edge(i + n, j + n);
            }
            if (yi - yj).abs() < d {
                graph.add_edge(i + n, j);
                graph.add_edge(j + n, i);
            }
        }
    }

    let edges = graph.scc();

    let mut use_x: Vec<Option<bool>> = vec![None; n];
    for edge in edges.into_iter().rev() {
        // 実現可能か確認
        let mut already_used: HashSet<usize> = HashSet::new();

        for &e in edge.iter() {
            let e: usize = if e >= n { e - n } else { e };
            if already_used.contains(&e) {
                println!("No");
                return;
            }
            already_used.insert(e);
        }

        // すでに保持している条件とぶつかるか？
        let mut skip = false;
        for &e in edge.iter() {
            if (e >= n) && (use_x[e - n] == Some(true)) {
                skip = true;
            }
            if (e < n) && (use_x[e] == Some(false)) {
                skip = true;
            }
        }
        if skip {
            continue;
        }

        // 状態を変更する

        for &e in edge.iter() {
            if (e >= n) && (use_x[e - n].is_none()) {
                use_x[e - n] = Some(false);
            } else if (e < n) && (use_x[e].is_none()) {
                use_x[e] = Some(true);
            }
        }
    }

    println!("Yes");
    for (i, is_x) in use_x.iter().enumerate() {
        if *is_x == Some(true) {
            println!("{}", xy[i].0);
        } else {
            println!("{}", xy[i].1);
        }
    }
}
