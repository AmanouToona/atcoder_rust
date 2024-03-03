use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
    }

    for a_ in a.iter() {
        let mut ans = Vec::new();
        for (i, &a__) in a_.iter().enumerate() {
            if a__ == 1 {
                ans.push(i + 1);
            }
        }

        let ans = ans
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        println!("{}", ans)
    }
}
