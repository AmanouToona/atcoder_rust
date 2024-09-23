use proconio::input;
use proconio::marker::Chars;
#[allow(non_snake_case)]
fn main() {
    input! {
        (_, Q): (usize, usize),
        S: Chars,
        XC: [(usize, char); Q],
    }

    let target = "ABC".to_string().chars().collect::<Vec<char>>();

    let mut s: Vec<char> = "..".to_string().chars().collect();
    s.extend(S);
    s.extend("..".to_string().chars().collect::<Vec<char>>());

    let mut cnt = 0;
    for left in 0..(s.len() - 2) {
        if s[left..left + 3] == target {
            cnt += 1;
        }
    }

    for (x, c) in XC.into_iter() {
        let x = x - 1 + 2; // ".." 分の 2 を足す

        for left in x..x + 3 {
            let left = left - 2;
            if s[left..left + 3] == target {
                cnt -= 1;
            }
        }

        s[x] = c;
        for left in x..x + 3 {
            let left = left - 2;

            if s[left..left + 3] == target {
                cnt += 1;
            }
        }
        println!("{cnt}");
    }
}
