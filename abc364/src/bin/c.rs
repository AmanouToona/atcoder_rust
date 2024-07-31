use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, X, Y): (usize, usize, usize),
    mut     A: [usize; N],
       mut  B: [usize; N],
    }

    A.sort();
    A = A.into_iter().rev().collect();

    B.sort();
    B = B.into_iter().rev().collect();

    let mut cnt_a = 0;
    let mut now_x = 0;
    for &a in A.iter() {
        cnt_a += 1;
        now_x += a;

        if now_x > X {
            break;
        }
    }

    let mut cnt_b = 0;
    let mut now_y = 0;
    for &b in B.iter() {
        cnt_b += 1;
        now_y += b;

        if now_y > Y {
            break;
        }
    }

    let ans = cnt_a.min(cnt_b);

    println!("{ans}");
}
