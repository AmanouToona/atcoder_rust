use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        (n, t): (usize, usize),
        ab: [(usize, usize); t],
    }

    let mut points: HashMap<usize, usize> = HashMap::new();
    let mut players: Vec<usize> = vec![0; n];

    points.insert(0, n);

    for &(a, b) in ab.iter() {
        let a = a - 1;

        let point_before = players[a];
        let point_after = point_before + b;

        players[a] = point_after;

        *points.entry(point_before).or_insert(0) -= 1;
        if points[&point_before] == 0 {
            points.remove(&point_before);
        }

        *points.entry(point_after).or_insert(0) += 1;

        println!("{}", points.len());
    }
}
