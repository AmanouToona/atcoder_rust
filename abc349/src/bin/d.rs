use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {
        (L, R): (i64, i64),
    }

    let mut ans = Vec::new();

    let mut l: i64 = L;
    loop {
        if l == R {
            break;
        }

        if l == 0 {
            let mut wi = 0;
            while (R - l) >= 2i64.pow(wi + 1) {
                wi += 1;
            }

            ans.push(format!("{} {}", l, l + 2i64.pow(wi)));
            l += 2i64.pow(wi);
            continue;
        }

        if l % 2 == 0 {
            let mut wi = 0;
            while 2i64.pow(wi + 1) <= l && 2i64.pow(wi + 1) <= R - l && l % 2i64.pow(wi + 1) == 0 {
                wi += 1;
            }

            ans.push(format!("{} {}", l, l + 2i64.pow(wi)));
            l += 2i64.pow(wi);
            // println!("{}", wi);

            continue;
        }

        ans.push(format!("{} {}", l, l + 1));
        l += 1;
    }

    println!("{}", ans.len());
    for a in ans {
        println!("{}", a);
    }
}
