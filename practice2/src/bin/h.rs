use ac_library::TwoSat;
use proconio::input;
fn main() {
    input! {
        (n, d): (usize, isize),
        xy: [(isize, isize); n],
    }

    let mut sat = TwoSat::new(n);

    for i in 0..xy.len() {
        for j in i..xy.len() {
            if i == j {
                continue;
            }

            let (xi, yi) = xy[i];
            let (xj, yj) = xy[j];

            if (xi - xj).abs() < d {
                sat.add_clause(i, true, j, true);
            }
            if (xi - yj).abs() < d {
                sat.add_clause(i, true, j, false);
            }
            if (yi - xj).abs() < d {
                sat.add_clause(i, false, j, true);
            }
            if (yi - yj).abs() < d {
                sat.add_clause(i, false, j, false);
            }
        }
    }

    if !sat.satisfiable() {
        println!("No");
        return;
    }

    println!("Yes");
    for (i, &x) in sat.answer().iter().enumerate() {
        if x {
            println!("{}", xy[i].1);
        } else {
            println!("{}", xy[i].0);
        }
    }
}
