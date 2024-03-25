use proconio::input;
fn main() {
    input! {
        s: String,
    }

    let mut ans = Vec::new();

    let mut start = false;
    for s_ in s.chars() {
        if s_ == '|' {
            start = !start;
            continue;
        }

        if start {
            continue;
        }

        ans.push(s_);
    }

    let ans: String = ans.iter().collect();

    println!("{}", ans);
}
