use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]

struct Solve {
    ans: usize,
    map: Vec<Vec<char>>,
    fp: Vec<Vec<bool>>,
    H: usize,
    W: usize,
    K: usize,
}

#[allow(non_snake_case)]

impl Solve {
    fn new(H: usize, W: usize, K: usize, map: Vec<Vec<char>>) -> Self {
        let fp = vec![vec![false; W]; H];

        Solve {
            ans: 0,
            map,
            fp,
            H,
            W,
            K,
        }
    }

    fn dfs(&mut self, uh: usize, uw: usize, cnt: usize) {
        // eprintln!("{uh} {uw} {cnt}");
        if cnt == self.K {
            self.ans += 1;
            return;
        }

        for (dh, dw) in [
            (0usize, 1usize),
            (1usize, 0usize),
            (0usize, !0usize),
            (!0usize, 0usize),
        ]
        .iter()
        {
            let vh = uh.wrapping_add(*dh);
            let vw = uw.wrapping_add(*dw);
            // eprintln!("{vh} {vw}");

            if vh >= self.H {
                continue;
            }
            if vw >= self.W {
                continue;
            }

            if self.map[vh][vw] == '#' {
                continue;
            }

            if self.fp[vh][vw] == true {
                continue;
            }

            self.fp[vh][vw] = true;

            self.dfs(vh, vw, cnt + 1);

            self.fp[vh][vw] = false;
        }
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W, K): (usize, usize, usize),
        S: [Chars; H],
    }

    let origin = S.clone();
    let mut solve = Solve::new(H, W, K, S);

    for h in 0..H {
        for w in 0..W {
            if origin[h][w] == '#' {
                continue;
            }
            // eprintln!("nex");
            solve.fp[h][w] = true;
            solve.dfs(h, w, 0);
            solve.fp[h][w] = false;
        }
    }

    let ans = solve.ans;
    println!("{ans}");
}
