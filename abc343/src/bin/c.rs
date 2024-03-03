use proconio::input;
fn main() {
    input! {n: usize}

    let mut ans = '0'.to_string();
    for i in 1..=(10usize.pow(6)) {
        let ans_ = i.pow(3);

        if ans_ > n {
            break;
        }

        let ans_ = ans_.to_string();

        if ans_.chars().eq(ans_.chars().rev()) {
            ans = ans_;
        }
    }

    println!("{}", ans);
}
