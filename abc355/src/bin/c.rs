use proconio::input;

fn is_bingo(b: &Vec<Vec<bool>>, p: (usize, usize)) -> bool {
    let n = b.len();

    // 左斜め
    let mut bingo = true;
    for i in 0..n {
        if !b[i][i] {
            bingo = false;
            break;
        }
    }

    if bingo {
        return true;
    }

    // 右斜め
    let mut bingo = true;
    for i in 0..n {
        if !b[n - 1 - i][i] {
            bingo = false;
        }
    }

    if bingo {
        return true;
    }

    // p 横
    let mut bingo = true;
    for i in 0..n {
        if !b[p.0][i] {
            bingo = false
        }
    }

    if bingo {
        return true;
    }

    // p 縦
    let mut bingo = true;
    for i in 0..n {
        if !b[i][p.1] {
            bingo = false;
        }
    }
    if bingo {
        return true;
    }

    false
}

#[allow(non_snake_case)]
fn main() {
    input! {
        (N, T): (usize, usize),
        A: [usize; T],
    }

    let mut b = vec![vec![false; N]; N];

    for (i, &a) in A.iter().enumerate() {
        let a = a - 1;
        let r = a / N;
        let c = a - r * N;

        // println!("{} {} ", r, c);

        b[r][c] = true;

        if is_bingo(&b, (r, c)) {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1");
}
