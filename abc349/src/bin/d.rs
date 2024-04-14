use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (L, R): (usize, usize),
    }

    let mut ans = Vec::new();

    let mut l = L;
    loop {
        if l == R {
            break;
        }
        let mut wi = 0;
        while 2usize.pow(wi + 1) <= R - l && l % 2usize.pow(wi + 1) == 0 {
            wi += 1;
        }

        ans.push(format!("{} {}", l, l + 2usize.pow(wi)));
        l += 2usize.pow(wi);
    }

    println!("{}", ans.len());
    for a in ans {
        println!("{}", a);
    }
}
