use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M, K): (usize, usize, usize),
        A: [usize; N],
    }

    if M >= N {
        let ans = vec![0; N].iter().join(" ");
        println!("{ans}");
        return;
    }

    let res = K - A.iter().sum::<usize>();
    eprintln!("res {res}");

    let mut Asort = A.clone();
    Asort.sort_by(|x, y| y.cmp(&x));

    let mut big = HashSet::new();
    let mut big_tot = 0;
    for a in Asort.iter().take(M) {
        big.insert(*a);
        big_tot += *a;
    }

    let mut ans: Vec<i64> = Vec::new();
    for a in A.iter() {
        let b = (a + big_tot + res) / (M + 1);
        eprintln!("rem {}", (a + big_tot + res) % (M + 1));

        if b <= *a {
            ans.push(0);
            continue;
        }

        let mut req = b - a;

        if (a + big_tot + res) % (M + 1) >= M {
            req += 1;
        }

        if req > res {
            ans.push(-1);
        } else {
            ans.push(req.try_into().unwrap());
        }
    }

    let ans: String = ans.iter().join(" ");
    println!("{ans}");
}
