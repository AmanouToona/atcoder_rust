use itertools::Itertools;
use proconio::input;
use std::collections::BTreeMap;
#[allow(non_snake_case)]
fn main() {
    input! {
        point : [usize; 5],
    }

    let problems: Vec<usize> = vec![0, 1, 2, 3, 4];

    let mut names: Vec<Vec<usize>> = Vec::new();

    for i in 1..=5 {
        for name in problems.iter().combinations(i) {
            let mut a: Vec<usize> = Vec::new();

            for n in name.iter() {
                a.push((**n).clone());
            }
            a.sort();
            names.push(a);
        }
    }

    let mut point_names: BTreeMap<usize, Vec<String>> = BTreeMap::new();

    for name in names.iter() {
        let mut get_point = 0;
        for n in name.iter() {
            get_point += point[*n];
        }

        let mut name = name.clone();
        name.sort();

        let name: String = name
            .iter()
            .map(|x| ('A' as u8 + *x as u8) as char)
            .collect();

        point_names
            .entry(get_point)
            .or_insert(Vec::new())
            .push(name);
    }

    for (_, v) in point_names.iter().rev() {
        let mut ans = v.clone();
        ans.sort();
        for a in ans.iter() {
            println!("{a}");
        }
    }
}
