use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        (H, W): (usize, usize),
        (mut sh, mut sw): (usize, usize),
        S: [Chars; H],
        X: String,
    }

    sh -= 1;
    sw -= 1;

    let mut uh = sh;
    let mut uw = sw;

    for x in X.chars() {
        let mut vh = 0;
        let mut vw = 0;
        if x == 'L' {
            vh = uh;
            vw = uw.wrapping_add(!0);
        } else if x == 'R' {
            vh = uh;
            vw = uw + 1;
        } else if x == 'U' {
            vh = uh.wrapping_add(!0);
            vw = uw;
        } else if x == 'D' {
            vh = uh + 1;
            vw = uw;
        } else {
            println!("wrong");
        }

        if vh >= H || vw >= W {
            continue;
        }

        if S[vh][vw] == '#' {
            continue;
        }

        uh = vh;
        uw = vw;
    }

    println!("{} {}", uh + 1, uw + 1);
}
