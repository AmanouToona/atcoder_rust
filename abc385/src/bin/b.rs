use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W, X, Y): (usize, usize, usize, usize),
        S: [Chars; H],
        T: Chars,
    }

    let mut ux = X - 1;
    let mut uy = Y - 1;

    let mut used = HashSet::new();

    for &t in T.iter() {
        let (dx, dy) = match t {
            'U' => (!0, 0),
            'D' => (1, 0),
            'L' => (0, !0),
            'R' => (0, 1),
            _ => panic!("oop!!"),
        };

        let vx = ux.wrapping_add(dx);
        let vy = uy.wrapping_add(dy);

        // eprintln!("{ux} {uy} {vx} {vy}");

        if vx >= H || vy >= W {
            continue;
        }

        if S[vx][vy] == '#' {
            continue;
        }

        if S[vx][vy] == '@' {
            used.insert((vx, vy));
        }

        ux = vx;
        uy = vy;
    }

    ux += 1;
    uy += 1;
    let cnt = used.len();
    println!("{ux} {uy} {cnt}");
}
