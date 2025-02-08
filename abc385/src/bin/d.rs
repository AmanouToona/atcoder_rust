use proconio::input;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::ops::Bound::{Included, Unbounded};

#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M, sx, sy): (usize, usize, i128, i128),
        XY: [(i128, i128); N],
        DC: [(char, i128); M],
    }

    let mut x_iny: HashMap<i128, BTreeSet<i128>> = HashMap::new();
    let mut y_inx: HashMap<i128, BTreeSet<i128>> = HashMap::new();

    for &(x, y) in XY.iter() {
        (*x_iny.entry(y).or_insert(BTreeSet::new())).insert(x);
        (*y_inx.entry(x).or_insert(BTreeSet::new())).insert(y);
    }

    let mut ux = sx;
    let mut uy = sy;
    let mut cnt = 0;
    for &(d, c) in DC.iter() {
        match d {
            'U' => {
                uy += c;

                // y なめ
                if !y_inx.contains_key(&ux) {
                    continue;
                }

                let mut ys = Vec::new();
                for &y in y_inx[&ux].range((Included(&(uy - c)), Unbounded)) {
                    if y > uy {
                        break;
                    }
                    cnt += 1;
                    ys.push(y);
                }

                // 削除
                for dely in ys.iter() {
                    y_inx.get_mut(&ux).map(|val| val.remove(dely));
                    x_iny.get_mut(dely).map(|val| val.remove(&ux));
                }
            }

            'D' => {
                uy -= c;

                // y なめ
                if !y_inx.contains_key(&ux) {
                    continue;
                }

                let mut ys = Vec::new();
                for &y in y_inx[&ux].range((Included(&uy), Unbounded)) {
                    if y > uy + c {
                        break;
                    }
                    cnt += 1;
                    ys.push(y);
                }

                // 削除
                for dely in ys.iter() {
                    y_inx.get_mut(&ux).map(|val| val.remove(dely));
                    x_iny.get_mut(dely).map(|val| val.remove(&ux));
                }
            }

            'L' => {
                ux -= c;

                // x なめ
                if !x_iny.contains_key(&uy) {
                    continue;
                }

                let mut xs = Vec::new();
                for &x in x_iny[&uy].range((Included(&ux), Unbounded)) {
                    if x > ux + c {
                        break;
                    }
                    cnt += 1;
                    xs.push(x);
                }

                // 削除
                for delx in xs.iter() {
                    x_iny.get_mut(&uy).map(|val| val.remove(delx));
                    y_inx.get_mut(delx).map(|val| val.remove(&uy));
                }
            }
            'R' => {
                ux += c;

                // x なめ
                if !x_iny.contains_key(&uy) {
                    continue;
                }

                let mut xs = Vec::new();
                for &x in x_iny[&uy].range((Included(&(ux - c)), Unbounded)) {
                    if x > ux {
                        break;
                    }
                    cnt += 1;
                    xs.push(x);
                }

                // 削除
                for delx in xs.iter() {
                    x_iny.get_mut(&uy).map(|val| val.remove(delx));
                    y_inx.get_mut(delx).map(|val| val.remove(&uy));
                }
            }
            _ => panic!("ooop"),
        }
    }
    println!("{ux} {uy} {cnt}");
}
