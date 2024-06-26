// https://atcoder.jp/contests/abc344/tasks/abc344_b
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    let mut A = Vec::new();
    loop {
        input! {a: usize}
        A.push(a);

        if a == 0 {
            break;
        }
    }

    A.reverse();

    for a in A {
        println!("{}", a);
    }
}
