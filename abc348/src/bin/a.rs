use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize
    }

    let mut ans = Vec::new();

    for i in 1..=N {
        if i % 3 == 0 {
            ans.push('x');
        } else {
            ans.push('o');
        }
    }

    let ans: String = ans.iter().collect();
    println!("{}", ans);
}
