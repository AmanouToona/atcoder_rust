use amplify::confinement::Collection;
use proconio::input;
use std::collections::HashSet;

struct Solve {
    groundy: Vec<Vec<Option<usize>>>,
}

impl Solve {
    fn new() -> Self {
        let mut groundy = vec![vec![None; 1400]; 1400];
        groundy[0][1] = Some(0usize);
        groundy[0][0] = Some(0usize);

        Solve { groundy }
    }

    fn get(&mut self, w: usize, b: usize) -> usize {
        if let Some(v) = self.groundy[w][b] {
            return v;
        }

        let mut stock = HashSet::new();
        for k in 1..=(b / 2) {
            stock.push(self.get(w, b - k));
        }
        if w > 0 {
            stock.push(self.get(w - 1, b + w));
        }

        let mut res = usize::MAX;
        for i in 0..1400 {
            if !stock.contains(&i) {
                res = i;
                break;
            }
        }

        self.groundy[w][b] = Some(res);

        res
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        W: [usize; N],
        B: [usize; N],
    }

    let mut groundy = Solve::new();

    let mut g = 0;

    for (&w, &b) in W.iter().zip(B.iter()) {
        g ^= groundy.get(w, b);
    }

    if g == 0 {
        println!("Second");
    } else {
        println!("First")
    }
}
