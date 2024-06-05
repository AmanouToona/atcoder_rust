use im_rc::{HashMap, HashSet};
use proconio::input;

struct Solve {
    ab: Vec<(usize, usize)>,
    n: usize,
    state: HashMap<usize, bool>, // 残っているカード, この状態で手番が回ってきたときに勝つか?
}

#[allow(non_snake_case)]
impl Solve {
    fn new(AB: Vec<(usize, usize)>) -> Self {
        let n = AB.len();
        let state = HashMap::new();

        Solve { ab: AB, n, state }
    }

    fn is_win(&mut self, remain: usize) -> bool {
        if remain == 0 {
            return false;
        }

        if self.state.contains_key(&remain) {
            return *self.state.get(&remain).unwrap();
        }

        let mut can: HashSet<(usize, usize)> = HashSet::new(); // 取り除けるカードの組
        for i in 0..self.n {
            if 1 << i & remain == 0 {
                continue;
            }

            for j in 0..self.n {
                if j == i {
                    continue;
                }
                if 1 << j & remain == 0 {
                    continue;
                }

                if self.ab[i].0 == self.ab[j].0 || self.ab[i].1 == self.ab[j].1 {
                    can.insert((i, j));
                }
            }
        }

        for &(i, j) in can.iter() {
            if !self.is_win(remain - (1 << i) - (1 << j)) {
                self.state.insert(remain, true);
                return true;
            }
        }

        self.state.insert(remain, false);

        false
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        AB: [(usize, usize); N],
    }

    let ta = "Takahashi";
    let ao = "Aoki";

    let mut solve = Solve::new(AB);
    let remain = (1 << N) - 1;

    if solve.is_win(remain) {
        println!("{}", ta);
    } else {
        println!("{}", ao);
    }
}
