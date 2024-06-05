use proconio::input;
#[allow(non_snake_case)]
fn main() {
    input! {(A, B): (usize, usize),}

    if A == B {
        println!("-1");
        return;
    }

    for i in 1..=3 {
        if i != A && i != B {
            println!("{}", i);
            return;
        }
    }
}
