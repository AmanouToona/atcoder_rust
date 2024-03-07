use ac_library::SccGraph;
use proconio::input;
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
    let edge = edges.iter().rev().next();

    match edge {
        Some(edge) => {
            let mut use_x: Vec<Option<bool>> = vec![None; n];

            for &e in edge.iter() {
                if e >= n {
                    let e = e - n;
                    if use_x[e] == Some(true) {
                        println!("No");
                        return;
                    }
                    use_x[e] = Some(false);
                } else {
                    if use_x[e] == Some(false) {
                        println!("No");
                        return;
                    }
                    use_x[e] = Some(true);
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
        None => {
            println!("Yes");
            for (x, _) in xy.iter() {
                println!("{}", x);
            }
        }
    }
}
